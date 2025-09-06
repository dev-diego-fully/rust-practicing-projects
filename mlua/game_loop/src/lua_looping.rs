//! This module contains the core logic for running a Lua-based game loop.
//! It handles loading user scripts, managing the game loop lifecycle,
//! and injecting necessary functions into the Lua environment.
//!
use crate::timer::Timer;
use mlua::prelude::*;

use std::fs;
use std::sync::{Arc, Mutex};

/// Loads a Lua script from a file and executes it in the given Lua state.
///
/// # Arguments
/// * `lua` - A reference to the Lua state.
/// * `path` - The file path to the user's Lua script.
///
/// # Returns
/// A `Result` indicating success or a `String` with an error message.
pub(crate) fn load_user_script(lua: &Lua, path: &str) -> Result<(), String> {
    match fs::read_to_string(path) {
        Ok(script) => lua
            .load(script)
            .exec()
            .map_err(|_| format!("Failed to run user script: {}", path)),
        Err(_) => Err(format!("Failed to load user script: {}", path)),
    }
}

/// Runs the main game loop, which continuously calls the Lua `update` function.
///
/// This function retrieves the `set_up` and `update` Lua functions, injects a
/// `stop` function for loop control, and enters a `while` loop that calls
/// the `update` function on each turn with the delta time.
///
/// # Arguments
/// * `lua` - A reference to the Lua state.
///
/// # Returns
/// A `Result` indicating success or a `String` with an error message if the
/// loop fails to start or an error occurs during an update.
pub(crate) fn game_loop(lua: &Lua) -> Result<(), String> {
    let is_game_loop_active: Arc<Mutex<bool>> = Arc::new(Mutex::new(true));

    let set_up: LuaFunction = retrieves_lua_loop_function(lua, "set_up")?;
    let update: LuaFunction = retrieves_lua_loop_function(lua, "update")?;

    inject_loop_stopper(lua, &is_game_loop_active)?;

    if set_up.call::<()>(()).is_err() {
        return Err("Failed to set_up game loop.".to_string());
    }

    let mut stop_watch = Timer::new();

    while *is_game_loop_active.lock().unwrap() {
        let delta = stop_watch.turn();
        if update.call::<()>(delta).is_err() {
            return Err("Failed on game loop update.".to_string());
        }
    }

    if lua.globals().set("stop", LuaNil).is_err() {
        return Err("An error occured in loop shutdown".to_string());
    }

    Ok(())
}

/// Retrieves a Lua function from the global environment.
///
/// # Arguments
/// * `lua` - A reference to the Lua state.
/// * `name` - The name of the function to retrieve.
///
/// # Returns
/// A `Result` with the `LuaFunction` or a `String` with an error message.
fn retrieves_lua_loop_function(lua: &Lua, name: &str) -> Result<LuaFunction, String> {
    lua.globals()
        .get(name)
        .map_err(|_| format!("Failed to retrieves lua function {}", name))
}

/// Injects a `stop` function into the Lua global environment.
///
/// This function is used to gracefully stop the main game loop from within
/// the Lua script.
///
/// # Arguments
/// * `lua` - A reference to the Lua state.
/// * `loop_controller` - An `Arc` to the boolean that controls the loop's state.
///
/// # Returns
/// A `Result` indicating success or a `String` with an error message.
fn inject_loop_stopper(lua: &Lua, loop_controller: &Arc<Mutex<bool>>) -> Result<(), String> {
    match create_loop_stopper_function(lua, loop_controller) {
        Ok(stopper) => lua
            .globals()
            .set("stop", stopper)
            .map_err(|_| "Failed to inject loop function stopper".to_string()),
        Err(msg) => Err(msg),
    }
}

/// Creates a Lua function that sets the loop controller boolean to `false`.
///
/// # Arguments
/// * `lua` - A reference to the Lua state.
/// * `loop_controller` - An `Arc` to the boolean that controls the loop's state.
///
/// # Returns
/// A `Result` with the new `LuaFunction` or a `String` with an error message.
fn create_loop_stopper_function(
    lua: &Lua,
    loop_controller: &Arc<Mutex<bool>>,
) -> Result<LuaFunction, String> {
    let lua_game_loop_verifier = loop_controller.clone();
    lua.create_function_mut(move |_, ()| {
        let mut cond = lua_game_loop_verifier.lock().unwrap();
        *cond = false;
        Ok(())
    })
    .map_err(|_| "Failed do create loop stopper funciton.".to_owned())
}