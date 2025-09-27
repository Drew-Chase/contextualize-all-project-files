use clap::Parser;
use log::*;
use tokio::io::AsyncWriteExt;

mod cli_arguments;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = cli_arguments::CLIArguments::parse();
    pretty_env_logger::env_logger::builder()
        .filter_level(match args.verbose {
            true => LevelFilter::Debug,
            false => LevelFilter::Info,
        })
        .format_timestamp(None)
        .init();
    let stopwatch = std::time::Instant::now();
    info!("Starting processing with args: {:?}", args);
    let absolute_input = std::fs::canonicalize(&args.input)?;
    let paths = walkdir::WalkDir::new(args.input)
        .into_iter()
        .filter_map(|e| {
            if let Ok(entry) = e {
                let is_dir = entry.path().is_dir();
                let relative_path = entry.path().strip_prefix(absolute_input.clone()).unwrap_or(entry.path()).to_string_lossy();
                if let Some(ignored_dirs) = &args.ignored_directories
                    && ignored_dirs.iter().any(|d| regex::Regex::new(d.trim()).is_ok_and(|r| r.is_match(&relative_path)))
                {
                    debug!("Ignoring directory {:?}", entry.path());
                    return None;
                }
                if let Some(ignored_files) = &args.ignored_files
                    && !is_dir
                    && ignored_files.iter().any(|f| regex::Regex::new(f).is_ok_and(|r| r.is_match(&relative_path)))
                {
                    debug!("Ignoring file {:?}", entry.path());
                    return None;
                }

                if let Some(extension) = entry.path().extension() {
                    let ext_str = extension.to_str().unwrap_or("").to_lowercase();
                    if let Some(blacklist) = &args.blacklist_extensions
                        && blacklist
                            .iter()
                            .any(|e| regex::Regex::new(&format!("{}$", e.trim_start_matches('.'))).map_or(false, |r| r.is_match(&ext_str)))
                    {
                        debug!("Ignoring file {:?} due to blacklist", entry.path());
                        return None;
                    }

                    if let Some(whitelist) = &args.whitelist_extensions
                        && whitelist
                            .iter()
                            .any(|e| regex::Regex::new(&format!("{}$", e.trim_start_matches('.'))).map_or(false, |r| !r.is_match(&ext_str)))
                    {
                        debug!("Ignoring file {:?} due to whitelist", entry.path());
                        return None;
                    }
                }

                return Some(entry.path().to_path_buf());
            }
            None
        })
        .collect::<Vec<_>>();

    info!("Found {} files to process", paths.len());
    if paths.is_empty() {
        warn!("No files to process, exiting");
        return Ok(());
    }
    debug!("Starting processing with {} threads", args.threads);

    let semaphore = std::sync::Arc::new(tokio::sync::Semaphore::new(args.threads));
    let text_content = std::sync::Arc::new(tokio::sync::Mutex::new(Vec::new()));
    let mut handles = Vec::new();
    for path in paths {
        let permit = semaphore.clone().acquire_owned().await?;
        let text_content = text_content.clone();
        let include_file_names = args.include_file_names;
        let handle = tokio::spawn(async move {
            let _permit = permit;
            if path.is_file() {
                match tokio::fs::read(&path).await {
                    Ok(data) => {
                        if let Ok(text) = String::from_utf8(data) {
                            if text.trim().is_empty() {
                                debug!("Skipping empty file {:?}", path);
                                return;
                            }
                            let mut content = text_content.lock().await;
                            let language = if let Some(ext) = path.extension().and_then(|ext| ext.to_str()).map(|ext| ext.to_lowercase()) {
                                ext
                            } else {
                                "".to_string()
                            }
                            .to_lowercase();
                            if include_file_names {
                                content.push(format!("{:?}:\n```{}\n{}\n```", path, language, text));
                            } else {
                                content.push(text);
                            }
                        } else {
                            warn!("File {:?} is not valid UTF-8, skipping", path);
                        }
                    }
                    Err(e) => {
                        error!("Failed to read file {:?}: {}", path, e);
                    }
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await?;
    }
    info!("All files processed, writing output to {}", args.output);
    let file = tokio::fs::File::create(&args.output).await?;
    let mut writer = tokio::io::BufWriter::new(file);
    let content = text_content.lock().await;
    for text in content.iter() {
        writer.write_all(text.as_bytes()).await?;
        writer.write_all(b"\n\n").await?;
    }
    writer.flush().await?;

    info!("Processing completed in {:.2?}", stopwatch.elapsed());
    Ok(())
}
