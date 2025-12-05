use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "disk-cleanup-tool")]
#[command(about = "Analyze and clean up disk space by identifying temporary directories", long_about = None)]
pub struct CliArgs {
    /// Directory path to analyze (defaults to current directory)
    #[arg(short, long)]
    pub path: Option<PathBuf>,

    /// Output CSV file path
    #[arg(short, long)]
    pub output_csv: Option<PathBuf>,

    /// Input CSV file path to load previous analysis
    #[arg(short, long)]
    pub input_csv: Option<PathBuf>,

    /// Show only temporary directories (node_modules, .venv, etc.)
    #[arg(short, long)]
    pub temp_only: bool,

    /// Launch interactive mode for selection and deletion
    #[arg(long)]
    pub interactive: bool,
}

pub fn parse_args() -> CliArgs {
    CliArgs::parse()
}
