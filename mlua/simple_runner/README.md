# Simple Lua Runner

This project is a small, personal application built to serve as a **host for embedded Lua scripts**. Its purpose is to demonstrate a basic setup for creating a command-line tool in Rust that can load and execute Lua code from a file.

The application uses:
-   `clap` for parsing command-line arguments.
-   `mlua` to embed a Lua interpreter and execute the user-provided script.

This project is an exercise in integrating a scripting language into a native Rust application, showing how to create a simple runner that extends its functionality with Lua.