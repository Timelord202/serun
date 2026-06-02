#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![expect(rustdoc::missing_crate_level_docs)]

use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;
use debug::{debugger, suites};

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
        test_suite: AvailableTests,

        /// Which cpu test to run
        #[arg(short, long)]
        instruction: Option<u8>
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
        Some(Commands::Test { test_suite: AvailableTests::Cpu, instruction }) => {
            if let Some(hex) = instruction {
                suites::cpu::run_one_test(*hex);
            } else {
                suites::cpu::run_all_tests();
            }
        },
        None => {}
    };
}