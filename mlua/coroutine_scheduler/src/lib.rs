//! This crate provides a cooperative scheduler for Lua coroutines,
//! with different task queueing policies.
//!
//! It exposes a Lua module that allows a user to create and manage
//! task schedulers directly from Lua scripts.
//!
mod tasks;
mod scheduler;
pub(crate) mod task_list;

use mlua::prelude::*;

/// The entry point for the Lua C module.
///
/// This function is called by the `mlua` framework when the module is
/// loaded from Lua. It returns a table containing factory functions
/// for creating different types of schedulers.
#[mlua::lua_module]
fn scheduler_core(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    
    exports.set("fifo", lua.create_function(crate::scheduler::fifo)?)?;
    exports.set("lottery", lua.create_function(crate::scheduler::lottery)?)?;

    Ok(exports)
}