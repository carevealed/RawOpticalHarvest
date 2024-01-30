use clap::Parser;
use clap_verbosity_flag::Verbosity;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args
{
    /// Path to the CSV file we want to process.
    pub csv_path: String,

    /// Run the program with extra diagnostic output.
    #[command(flatten)]
    pub verbose: Verbosity,
}
