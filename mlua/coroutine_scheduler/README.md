# Rust Coroutine Scheduler for Lua

This crate provides a cooperative scheduler for Lua coroutines, implemented in Rust. It offers different task queueing policies to manage and execute tasks.

### Purpose

This project was developed as a hands-on learning exercise to explore advanced concepts in Rust and its interoperability with Lua. The primary goal was to create a robust and flexible cooperative scheduler that could emulate professional thread scheduling behaviors, while providing a practical, high-performance solution for managing Lua coroutines without manual yield control.

### Features

- **Cooperative Multitasking**: Schedule and manage multiple Lua coroutines.
- **Pluggable Scheduling Policies**: Includes built-in support for two scheduling policies:
    - **FIFO (First-In, First-Out)**: Tasks are executed in the order they are added.
    - **Lottery Scheduling**: Tasks are selected based on a weighted random "lottery" system, where a task's priority determines its chance of being chosen.
- **Lua Integration**: Exposes a Lua module that allows you to create and control schedulers directly from Lua scripts.

### Build and Usage

To build the project for a specific Lua version (e.g., `lua54`), use the provided script:

```bash
./build.sh lua54
```

This command generates a Lua C module (core.so) and a Lua stub file (init.lua), which are placed in the scheduler directory. You can then use the module from your Lua scripts:

```lua
local scheduler = require("scheduler")

local my_scheduler = scheduler.lottery()

-- Create and spawn tasks
my_scheduler:spawn_task(function()
    print("Task 1 running")
    coroutine.yield()
    print("Task 1 resumed")
end, 2)

my_scheduler:spawn_task(function()
    print("Task 2 running")
    coroutine.yield()
    print("Task 2 resumed")
end, 1)

-- Run until all tasks are completed
my_scheduler:run()
```