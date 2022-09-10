mod lua_exports;
mod file;
mod directory;
mod path_filter;
mod error;

#[no_mangle]
pub extern "C" fn luaopen_itb_io(lua_state: *mut mlua::lua_State) -> i32 {
    // Leak the Lua purposefully because it's supposed to live for the duration of the program.
    // It should be owned by the game, so as a client DLL, we can assume it's truly 'static.
    let lua = unsafe { mlua::Lua::init_from_ptr(lua_state) }.into_static();

    let export = lua_exports::init(&lua).expect("Failed to initialize module export table");
    lua.globals().set("itb_io", export).unwrap();

    0
}
