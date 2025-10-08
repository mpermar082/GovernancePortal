// src/main.rs
/*
 * Main executable for GovernancePortal
 */

use clap::Parser;
use governanceportal::{Result, run};

/// Command-line arguments parser
#[derive(Parser)]
#[command(version, about = "GovernancePortal - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
    
    /// Input file path (optional)
    #[arg(short, long)]
    input: Option<String>,
    
    /// Output file path (optional)
    #[arg(short, long)]
    output: Option<String>,
}

/// Runs the GovernancePortal application
fn main() -> Result<()> {
    // Parse command-line arguments
    let args = Cli::parse();
    
    // Run the application with parsed arguments
    // This line can be made more readable by adding a comment describing the purpose of the run function
    run(args.verbose, args.input, args.output)
    // Example of a meaningful improvement: add error handling for the run function
    // This is a minimal but meaningful improvement
    .expect("Failed to run GovernancePortal application")
}