//! A simple command-line application that loads and executes a Lua script from a file.
//!
//! This program uses `clap` to parse command-line arguments and `mlua` to
//! interface with a Lua interpreter, providing a basic runner for user-provided scripts.
//!
use clap::{Parser, command};
use mlua::prelude::*;
use std::fs;

/// The main entry point of the application.
///
/// It parses command-line arguments to get the script file path, reads the
/// file content, and attempts to execute it as a Lua script. Any errors
/// in this process are printed to the console.
fn main() {
    let args = Args::parse();
    if let Err(msg) = file_content(&args.file).map(run_script) {
        println!("{}", msg);
    };
}

/// Reads the content of a file into a `String`.
///
/// # Arguments
///
/// * `path` - A string slice that holds the file path.
///
/// # Returns
///
/// A `Result` containing the file content as a `String` on success, or a
/// `String` with an error message on failure.
fn file_content(path: &str) -> Result<String, String> {
    match fs::read_to_string(path) {
        Ok(v) => Ok(v),
        Err(_) => Err(format!("Failed to load file: {}", path)),
    }
}

/// Executes a Lua script within a new Lua state.
///
/// # Arguments
///
/// * `script` - The Lua script content as a `String`.
///
/// # Returns
///
/// A `Result` indicating success or a static string slice with an error message
/// if the script execution fails.
fn run_script(script: String) -> Result<(), &'static str> {
    Lua::new()
        .load(script)
        .exec()
        .map_err(|_| "Failed to run lua script.")
}

/// A simple application to run a Lua script from a file.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The path to the Lua script to be executed.
    #[arg(help = "Path of lua script.")]
    file: String,
}