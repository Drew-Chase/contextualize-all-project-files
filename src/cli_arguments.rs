use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "Contextualize All Files Tool", about, author, version, propagate_version = true)]
pub struct CLIArguments {
    #[arg(short, long, help = "Input directory to process", default_value = "./")]
    pub input: String,
    #[arg(long="whitelist", help = "A comma separated list of file extensions or regex patterns to include (e.g. --whitelist '.txt, .md, .*doc'), if none is set all utf-8/16 files will be included", num_args=0.., value_delimiter = ','
    )]
    pub whitelist_extensions: Option<Vec<String>>,
    #[arg(long="blacklist", help = "A comma separated list of file extensions or regex patterns to exclude (e.g. --blacklist '.log, .bin, .*temp'), if none is set no files will be excluded", num_args=0.., value_delimiter = ','
    )]
    pub blacklist_extensions: Option<Vec<String>>,
    #[arg(long="ignore-dir", help = "A comma separated list of directory names or regex patterns to ignore (e.g. --ignore-dir 'node_modules, .git, .*cache'), if none is set no directories will be ignored", num_args=0.., value_delimiter = ','
    )]
    pub ignored_directories: Option<Vec<String>>,
    #[arg(long="ignore-file", help = "A comma separated list of file names or regex patterns to ignore (e.g. --ignore-file 'README.md, .DS_Store, .*test.*'), if none is set no files will be ignored", num_args=0.., value_delimiter = ','
    )]
    pub ignored_files: Option<Vec<String>>,
    #[arg(short, long, help = "Number of threads to use", default_value_t = 4)]
    pub threads: usize,
    #[arg(short = 'f', long, help = "Include file names in the output", default_value_t = false)]
    pub include_file_names: bool,
    #[arg(short, long, help = "Enable verbose logging", default_value_t = false)]
    pub verbose: bool,
    #[arg(short, long, help = "Output file for results")]
    pub output: String,
}
