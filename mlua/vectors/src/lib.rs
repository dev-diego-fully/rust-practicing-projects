mod adapter;
mod vector;

use adapter::LuaVectorAdapter;
use mlua::prelude::*;

/// The entry point for the Lua module, exposing the `Vector` type.
///
/// This function is called when the Lua script requires the module. It creates
/// a table containing the module's public interface, which in this case is
/// the `Vector.new` constructor.
#[mlua::lua_module]
fn vector(lua: &Lua) -> LuaResult<LuaTable> {
    let module = lua.create_table()?;

    module.set("new", lua.create_function(LuaVectorAdapter::new)?)?;
    module.set("of", lua.create_function(LuaVectorAdapter::of)?)?;

    Ok(module)
}

impl LuaUserData for LuaVectorAdapter {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("get", Self::get);
        methods.add_method("is_same", Self::is_same);

        methods.add_method_mut("set", Self::set);
        methods.add_method_mut("push", Self::push);
        methods.add_method_mut("pop", Self::pop);

        methods.add_meta_method("__index", Self::index);
        methods.add_meta_method("__len", Self::len);
        methods.add_meta_method("__eq", Self::equals);

        methods.add_meta_method_mut("__newindex", Self::newindex);
    }
}
