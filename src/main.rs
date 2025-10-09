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
    
    // Run the GovernancePortal application with parsed arguments
    // This line can be made more readable by adding a comment describing the purpose of the run function
    match run(args.verbose, args.input, args.output) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Failed to run GovernancePortal application: {}", e)),
    }
}