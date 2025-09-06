# Simple Game Loop with Lua

This project is a personal exercise that demonstrates how to create a basic game loop in Rust with an embedded Lua interpreter. It provides a simple framework for running a Lua script as the core logic of a game or simulation.

The application handles the main game loop, which continuously calls a user-defined `update` function written in Lua, passing the delta time between frames. It also injects a `stop` function into the Lua environment, allowing the script to gracefully terminate the loop.

This project showcases the integration of Rust with Lua for developing a flexible application structure where the core engine is written in Rust, and the game logic is handled by a separate Lua script.