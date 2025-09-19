//! A module that provides a Lua-facing `Vector` type, implemented in Rust.
//!
//! This module allows Lua scripts to create and manipulate a dynamic array
//! that is backed by a Rust `Vec<mlua::LuaValue>`, offering efficient
//! operations for a vector-like data structure.
//!
//! The `Vector` type implements the `mlua::LuaUserData` trait, exposing its
//! methods and meta-methods to the Lua runtime.
use mlua::prelude::*;

/// A Rust-backed vector for storing Lua values.
///
/// This struct holds the actual data in a `Vec<LuaValue>` and exposes a
/// safe, idiomatic API for common vector operations.
struct Vector {
    /// The internal vector that stores the Lua values.
    inner: Vec<LuaValue>,
}

impl Vector {
    /// Creates a new, empty `Vector` instance.
    fn new() -> Self {
        Self { inner: Vec::new() }
    }

    /// Appends a new value to the end of the vector.
    ///
    /// This method checks for potential integer overflow before pushing the value
    /// and returns a boolean indicating success or failure.
    ///
    /// # Arguments
    ///
    /// * `value` - The `LuaValue` to be pushed into the vector.
    fn push(&mut self, value: LuaValue) -> LuaValue {
        if self.inner.len() as LuaInteger <= LuaInteger::MAX {
            self.inner.push(value);
            return LuaValue::Boolean(true);
        }
        LuaValue::Boolean(false)
    }

    /// Removes and returns the last value from the vector.
    ///
    /// Returns `None` if the vector is empty.
    fn pop(&mut self) -> Option<LuaValue> {
        self.inner.pop()
    }

    /// Returns a cloned reference to the value at a specific index.
    ///
    /// The index is 1-based, consistent with Lua's array indexing.
    /// Returns `None` if the index is out of bounds or non-positive.
    ///
    /// # Arguments
    ///
    /// * `index` - The 1-based index of the value to retrieve.
    fn get(&self, index: LuaInteger) -> Option<LuaValue> {
        if index <= 0 {
            return None;
        }
        let zero_base_index = (index - 1) as usize;
        self.inner.get(zero_base_index).cloned()
    }

    /// Sets the value at a specific index.
    ///
    /// The index is 1-based, consistent with Lua's array indexing.
    /// Returns `true` on success, or `false` if the index is out of bounds.
    ///
    /// # Arguments
    ///
    /// * `index` - The 1-based index of the value to modify.
    /// * `new_value` - The new `LuaValue` to set.
    fn set(&mut self, index: LuaInteger, new_value: LuaValue) -> bool {
        if index <= 0 {
            return false;
        }
        let zero_based_index = (index - 1) as usize;

        if zero_based_index >= self.inner.len() {
            return false;
        }
        self.inner[zero_based_index] = new_value;
        true
    }

    /// Returns the number of elements in the vector.
    fn len(&self) -> LuaInteger {
        self.inner.len() as LuaInteger
    }
}

/// A block that contains the FFI (Foreign Function Interface) wrappers
/// for the `Vector` methods, designed to be called from Lua.
impl Vector {
    /// The Lua-facing constructor for `Vector`.
    fn lua_new(_: &Lua, _: ()) -> LuaResult<Self> {
        Ok(Self::new())
    }

    /// The Lua-facing wrapper for the `push` method.
    fn lua_push(_: &Lua, this: &mut Self, (value,): (LuaValue,)) -> LuaResult<LuaValue> {
        Ok(this.push(value))
    }

    /// The Lua-facing wrapper for the `pop` method.
    ///
    /// Returns `true, value` on success, or `false, nil` if the vector is empty.
    fn lua_pop(_: &Lua, this: &mut Self, _: ()) -> LuaResult<(LuaValue, LuaValue)> {
        match this.pop() {
            Some(value) => Ok((LuaValue::Boolean(true), value)),
            None => Ok((LuaValue::Boolean(false), LuaNil)),
        }
    }

    /// The Lua-facing wrapper for the `get` method.
    ///
    /// Returns the value at the specified index or `nil` if the index is invalid.
    fn lua_get(_: &Lua, this: &Self, (index,): (LuaInteger,)) -> LuaResult<LuaValue> {
        match this.get(index) {
            Some(value) => Ok(value),
            None => Ok(LuaNil),
        }
    }

    /// The Lua-facing wrapper for the `set` method.
    ///
    /// Sets the value at the specified index. This method handles out-of-bounds
    /// errors by raising a Lua runtime error.
    fn lua_set(_: &Lua, this: &mut Self, (index, value): (LuaInteger, LuaValue)) -> LuaResult<()> {
        if this.set(index, value) {
            return Ok(());
        }
        let msg = format!(
            "Index out of bounds => index {}, vector len: {}",
            index,
            this.len()
        );
        Err(mlua::Error::RuntimeError(msg))
    }

    /// The Lua-facing wrapper for the `len` method.
    fn lua_len(_: &Lua, this: &Self, _: ()) -> LuaResult<LuaInteger> {
        Ok(this.len())
    }
}

/// `LuaUserData` trait implementation that defines which methods are exposed to Lua.
///
/// This implementation links the Lua-facing functions (`lua_*`) to the Lua
/// runtime, making them accessible from a Lua script. It also uses
/// `add_meta_method` for `index` and `newindex` to enable syntactic sugar
/// like `vec[1]` and `vec[1] = "value"`.
impl LuaUserData for Vector {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_function("new", Self::lua_new);

        methods.add_method_mut("push", Self::lua_push);

        methods.add_method_mut("pop", Self::lua_pop);

        methods.add_method("get", Self::lua_get);
        methods.add_meta_method("index", Self::lua_get);

        methods.add_method_mut("set", Self::lua_set);
        methods.add_meta_method_mut("newindex", Self::lua_set);
        
        methods.add_method("len", Self::lua_len);
    }
}

/// The entry point for the Lua module, exposing the `Vector` type.
///
/// This function is called when the Lua script requires the module. It creates
/// a table containing the module's public interface, which in this case is
/// the `Vector.new` constructor.
#[mlua::lua_module]
fn vector_core(lua: &Lua) -> LuaResult<LuaTable> {
    let module = lua.create_table()?;

    module.set("new", Vector::new())?;

    Ok(module)
}