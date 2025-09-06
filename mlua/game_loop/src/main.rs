//! This is the main executable for a simple application that runs a Lua script
//! in a game loop context. It handles command-line argument parsing and
//! orchestrates the execution flow.
//!
mod lua_looping;
mod timer;

use clap::{Parser, command};
use mlua::prelude::*;

/// A simple application to run a Lua script in a game loop.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to main lua script
    file: String,
}

/// The main entry point of the application.
///
/// It parses command-line arguments to get the path to the Lua script,
/// initializes the Lua environment, loads and executes the user's script,
/// and then starts the main game loop. Errors are printed to the console.
fn main() {
    let lua = Lua::new();
    let _ = lua_looping::load_user_script(&lua, &Args::parse().file)
        .map(|_| lua_looping::game_loop(&lua))
        .inspect_err(|msg| println!("{}", msg));
}