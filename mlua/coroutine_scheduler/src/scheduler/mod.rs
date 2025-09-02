//! This module acts as a factory, exposing functions to create different
//! types of `LuaScheduler` instances for the Lua environment.
//!
mod lua;
mod rust;

use mlua::prelude::*;
use rust::LuaScheduler;
use crate::task_list::*;

/// A factory function that creates a new `LuaScheduler` using a `FIFOTaskList`.
pub(crate) fn fifo(_: &Lua, _: ()) -> LuaResult<LuaScheduler<FIFOTaskList>> {
    Ok(LuaScheduler::new(FIFOTaskList::new()))
}

/// A factory function that creates a new `LuaScheduler` using a `Lottery` task list.
pub(crate) fn lottery(_: &Lua, _:()) -> LuaResult<LuaScheduler<Lottery>> {
    Ok(LuaScheduler::new(Lottery::new()))
}