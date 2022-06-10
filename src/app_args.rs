use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct AppArgs {
    /// number of times to execute each algorithm
    #[clap(short, long)]
    runs: usize,

    /// File to save the csv output
    #[clap(short, long)]
    output_filename: String,
}

impl AppArgs {
    pub fn new() -> AppArgs {
        AppArgs::parse()
    }

    pub fn runs(&self) -> usize {
        self.runs
    }

    pub fn output_filename(&self) -> &str {
        &self.output_filename
    }
}
