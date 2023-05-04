use std::{
    ops::RangeBounds,
    path::{Path, PathBuf},
};

use clap::{command, Parser, Subcommand};
use lopdf::Document;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(short, long, action = clap::ArgAction::Count)]
    verbosity: u8,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Inspect {
        /// The path to the PDF file to inspect
        path: PathBuf,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Inspect { path } => {
            let document = Document::load(path)?;
            inspect(&document)
        }
    }
}

// fn inspect(path: &Path, page_range: impl RangeBounds<usize>) -> anyhow::Result<()> {
fn inspect(document: &Document) -> anyhow::Result<()> {
    let pages = document.get_pages();

    for page in pages.values() {
        let content = document.get_and_decode_page_content(*page)?;
        // make_page(p)?;
    }

    Ok(())
}
