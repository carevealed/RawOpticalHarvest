use clap::Parser;
use clap_verbosity_flag::Verbosity;

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about,
    long_about = "A tool to automate the process of collecting and storing \
                  California Revealed's digital assets"
)]
pub struct Cli
{
    /// Path to the CSV file we want to process.

    #[arg(value_name = "Input CSV")]
    pub csv_path: String,

    /// Output parent directory.
    #[arg(value_name = "Output Parent Directory")]
    pub output_parent_path: String,

    /// Device to use as ISO generation source.  If none is provided, the user
    /// will be prompted to select a device.
    #[arg(value_name = "ROM Device")]
    pub rom_device: Option<String>,

    /// Don't actually create or modify any files
    #[arg(long, short)]
    pub dry_run: bool,

    ///TODO: Run interactively
    // #[clap(long, short, action)]
    // pub interactive: bool,

    /// Run the program with extra diagnostic output.
    #[command(flatten)]
    pub verbose: Verbosity,
}
