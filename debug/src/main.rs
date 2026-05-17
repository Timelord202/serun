#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![expect(rustdoc::missing_crate_level_docs)]

use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;
use debug::debugger;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Run a test suite to verify functionality of components
    Test {
        /// The test suite to run.
        #[arg(short, long, value_enum)]
        test_suite: AvailableTests
    },
    /// Runs the debugger given a path to a .nes file
    Run {
        /// Path to .nes file
        #[arg(short, long, value_name = "FILE")]
        path: PathBuf,
    },
}

#[derive(Clone, ValueEnum)]
enum AvailableTests {
    Cpu
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Run { path }) => {
            // TODO display err message if debugger errors out
            let _ = debugger::run_debugger(path.to_owned());
        },
        Some(Commands::Test { test_suite }) => {
            match test_suite {
                AvailableTests::Cpu => {
                    println!("TODO: Still developing cpu test suite");
                }
            }
        },
        None => {}
    };
}

