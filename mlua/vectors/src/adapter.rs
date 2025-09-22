use mlua::prelude::*;

use crate::vector::LuaVector;

/// A type alias for the `Vector` struct, used for clarity in Lua bindings.
pub(super) type LuaVectorAdapter = Vector;

/// The adapter struct that wraps the core `LuaVector` and implements the Lua-facing logic.
///
/// This layer handles all interactions with the `mlua` library, including type
/// conversions and error handling, abstracting the core logic away from the FFI.
pub(super) struct Vector {
    /// The internal `LuaVector` that holds the actual data.
    vec: LuaVector,
}

impl LuaVectorAdapter {
    /// The Lua-facing constructor for creating a new `Vector`.
    ///
    /// It handles optional arguments for initial size and filler value, consistent
    /// with the Lua module's behavior.
    pub(super) fn new(
        _: &Lua,
        (size, filler): (Option<LuaInteger>, Option<LuaValue>),
    ) -> LuaResult<Self> {
        let vec_size = size.unwrap_or(0);
        let vec_filler = filler.unwrap_or(LuaNil);

        match LuaVector::new(vec_size, vec_filler) {
            Ok(vec) => Ok(Self { vec }),
            Err(msg) => Err(LuaError::RuntimeError(msg)),
        }
    }

    /// The Lua-facing constructor for creating a `Vector` from variadic arguments.
    ///
    /// It accepts any number of arguments and uses them to populate a new vector.
    pub(super) fn of(_: &Lua, values: LuaVariadic<LuaValue>) -> LuaResult<Self> {
        Ok(Self {
            vec: LuaVector::of(values.to_vec()),
        })
    }

    /// The Lua-facing `get` method.
    ///
    /// It returns a pair of values: a boolean presence flag and the value itself.
    /// This allows distinguishing between a `nil` that is present and a value that is out of bounds.
    pub(super) fn get(
        _: &Lua,
        this: &Self,
        (index,): (LuaInteger,),
    ) -> LuaResult<(LuaValue, LuaValue)> {
        Ok(match this.vec.get(index) {
            Some(value) => (LuaValue::Boolean(true), value),
            None => (LuaValue::Boolean(false), LuaNil),
        })
    }

    /// The Lua-facing `is_same` method, which checks for reference equality.
    ///
    /// It returns `true` only if the two `Vector` objects are the same instance.
    /// This is distinct from the `__eq` metamethod which checks for value equality.
    pub(super) fn is_same(_: &Lua, this: &Self, (other,): (LuaValue,)) -> LuaResult<LuaValue> {
        let lua_false = LuaValue::Boolean(false);

        let result_ud = match other {
            LuaValue::UserData(ud) => ud.borrow::<LuaVectorAdapter>(),
            _ => return Ok(lua_false),
        };

        match result_ud {
            Ok(ref_vec) => {
                let is_same = this.vec.is_same(&ref_vec.vec);
                Ok(LuaValue::Boolean(is_same))
            }
            Err(_) => Ok(lua_false),
        }
    }

    /// The Lua-facing `set` method.
    ///
    /// It returns a boolean indicating whether the operation succeeded. This method
    /// does not cause a Lua error if the index is out of bounds.
    pub(super) fn set(
        _: &Lua,
        this: &mut Self,
        (index, new_value): (LuaInteger, LuaValue),
    ) -> LuaResult<LuaValue> {
        let succeed = this.vec.set(index, new_value);

        Ok(LuaValue::Boolean(succeed))
    }

    /// The Lua-facing `push` method.
    ///
    /// Appends a new value to the end of the vector and returns `true` on success.
    pub(super) fn push(_: &Lua, this: &mut Self, (value,): (LuaValue,)) -> LuaResult<LuaValue> {
        Ok(LuaValue::Boolean(this.vec.push(value)))
    }

    /// The Lua-facing `pop` method.
    ///
    /// It removes and returns the last element, along with a presence flag.
    pub(super) fn pop(_: &Lua, this: &mut Self, _: ()) -> LuaResult<(LuaValue, LuaValue)> {
        match this.vec.pop() {
            Some(value) => Ok((LuaValue::Boolean(true), value)),
            None => Ok((LuaValue::Boolean(false), LuaNil)),
        }
    }

    /// The Lua-facing `__index` metamethod.
    ///
    /// It retrieves an element by index using the `[]` operator. Unlike the explicit `get`
    /// method, this metamethod causes a Lua runtime error if the index is out of bounds.
    pub(super) fn index(_: &Lua, this: &Self, (index,): (LuaInteger,)) -> LuaResult<LuaValue> {
        match this.vec.get(index) {
            Some(value) => Ok(value),

            None => {
                let msg = format!("Index out of bounds {}", index);
                let lua_err = LuaError::runtime(msg);
                Err(lua_err)
            }
        }
    }

    /// The Lua-facing `__len` metamethod.
    ///
    /// It is called when the length operator `#` is used on a `Vector` instance.
    pub(super) fn len(_: &Lua, this: &Self, _: ()) -> LuaResult<LuaInteger> {
        Ok(this.vec.len())
    }

    /// The Lua-facing `__eq` metamethod.
    ///
    /// It is called when the equality operator `==` is used. This method performs a
    /// deep comparison of the contents of two vectors.
    pub(super) fn equals(_: &Lua, this: &Self, (other,): (LuaValue,)) -> LuaResult<LuaValue> {
        let lua_false = LuaValue::Boolean(false);

        let result_ud = match other {
            LuaValue::UserData(ud) => ud.borrow::<LuaVectorAdapter>(),
            _ => return Ok(lua_false),
        };

        match result_ud {
            Ok(ref_vec) => {
                let equals = this.vec == ref_vec.vec;
                Ok(LuaValue::Boolean(equals))
            }
            Err(_) => Ok(lua_false),
        }
    }

    /// The Lua-facing `__newindex` metamethod.
    ///
    /// It is called when an element is assigned using the `[]` operator. Unlike the
    /// explicit `set` method, this metamethod causes a Lua runtime error if the
    /// index is out of bounds.
    pub(super) fn newindex(
        _: &Lua,
        this: &mut Self,
        (index, value): (LuaInteger, LuaValue),
    ) -> LuaResult<()> {
        if !this.vec.set(index, value) {
            let msg = format!("Index out of bounds {}", index);
            let lua_err = LuaError::runtime(msg);
            Err(lua_err)
        } else {
            Ok(())
        }
    }
}