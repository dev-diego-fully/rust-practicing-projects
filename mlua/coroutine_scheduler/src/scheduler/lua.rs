//! This module implements the `LuaUserData` trait for `LuaScheduler`,
//! making the scheduler's functionality accessible from a Lua script.
//!
use mlua::prelude::*;

use super::rust::LuaScheduler;
use crate::{task_list::TaskList};

impl<Tasks: TaskList + 'static> LuaScheduler<Tasks> {
    /// Wraps the `steps` method for use in Lua.
    ///
    /// It allows the scheduler to execute for a specified number of steps.
    ///
    /// # Arguments
    /// * `this` - The scheduler instance.
    /// * `steps` - The number of steps to execute. Defaults to 1.
    ///
    /// # Returns
    /// An empty `LuaResult` on success or a runtime error if the step count is not positive.
    fn lua_steps(_: &Lua, this: &mut Self, steps: Option<LuaInteger>) -> LuaResult<()> {
        let count = steps.unwrap_or(1);

        if count <= 0 {
            return Err(LuaError::runtime("Cant execute non positive steps count"));
        }

        this.steps(count);
        Ok(())
    }

    /// Wraps the `has_tasks` method for use in Lua.
    ///
    /// # Arguments
    /// * `this` - The scheduler instance.
    ///
    /// # Returns
    /// A `LuaValue::Boolean` indicating whether the scheduler has tasks.
    fn lua_has_tasks(_: &Lua, this: &Self, _: ()) -> LuaResult<LuaValue> {
        Ok(LuaValue::Boolean(this.has_tasks()))
    }

    /// Wraps the `run` method for use in Lua.
    ///
    /// # Arguments
    /// * `this` - The scheduler instance.
    ///
    /// # Returns
    /// An empty `LuaResult` on success.
    fn lua_run(_: &Lua, this: &mut Self, _: ()) -> LuaResult<()> {
        this.run();
        Ok(())
    }

    /// Wraps the `add_task` method for use in Lua.
    ///
    /// # Arguments
    /// * `this` - The scheduler instance.
    /// * `function` - The Lua function to be converted into a task.
    /// * `priority` - The priority of the task. Defaults to 1.
    ///
    /// # Returns
    /// An empty `LuaResult` on success or a runtime error if the priority is not positive.
    fn lua_spawn_task(
        lua: &Lua,
        this: &mut Self,
        (function, priority): (LuaFunction, Option<LuaInteger>),
    ) -> LuaResult<()> {
        let prior = priority.unwrap_or(1);

        if prior <= 0 {
            return Err(LuaError::runtime("Can't deal with non positive priority"));
        }

        let coroutine = lua.create_thread(function)?;

        this.add_task(coroutine, prior);
        Ok(())
    }
}

impl<Tasks: TaskList + 'static> LuaUserData for LuaScheduler<Tasks> {
    /// Defines the methods that will be exposed to Lua.
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("has_tasks", Self::lua_has_tasks);
        methods.add_method_mut("step", Self::lua_steps);
        methods.add_method_mut("run", Self::lua_run);
        methods.add_method_mut("spawn_task", Self::lua_spawn_task);
    }
}