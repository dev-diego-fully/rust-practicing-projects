use mlua::prelude::*;

use crate::vector::LuaVector;

pub(super) type LuaVectorAdapter = Vector;

pub(super) struct Vector {
    vec: LuaVector,
}

impl LuaVectorAdapter {
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

    pub(super) fn of(_: &Lua, values: LuaVariadic<LuaValue>) -> LuaResult<Self> {
        Ok(Self {
            vec: LuaVector::of(values.to_vec()),
        })
    }

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

    pub(super) fn set(
        _: &Lua,
        this: &mut Self,
        (index, new_value): (LuaInteger, LuaValue),
    ) -> LuaResult<LuaValue> {
        let succeed = this.vec.set(index, new_value);

        Ok(LuaValue::Boolean(succeed))
    }

    pub(super) fn push(_: &Lua, this: &mut Self, (value,): (LuaValue,)) -> LuaResult<LuaValue> {
        Ok(LuaValue::Boolean(this.vec.push(value)))
    }

    pub(super) fn pop(_: &Lua, this: &mut Self, _: ()) -> LuaResult<(LuaValue, LuaValue)> {
        match this.vec.pop() {
            Some(value) => Ok((LuaValue::Boolean(true), value)),
            None => Ok((LuaValue::Boolean(false), LuaNil)),
        }
    }

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

    pub(super) fn len(_: &Lua, this: &Self, _: ()) -> LuaResult<LuaInteger> {
        Ok(this.vec.len())
    }

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
