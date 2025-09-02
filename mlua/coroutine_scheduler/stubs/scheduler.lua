--- A module that provides cooperative schedulers for managing Lua coroutines.
--- It offers different task queueing strategies, such as FIFO and Lottery.
local scheduler = {};

--- Base class for all scheduler types.
--- Schedulers manage the execution of tasks (coroutines).
---@class Scheduler
---@field steps_count integer The total number of steps executed by the scheduler across all tasks.
local Scheduler = {}

--- Adds a new task (coroutine function) to the scheduler.
--- The task function should ideally yield periodically to allow the scheduler to
--- switch context and manage other tasks.
---@param task_fn fun() The coroutine function representing the task.
---@param priority? integer An optional priority value. Its interpretation depends on the scheduler type. For FIFO schedulers, this value is ignored.
function Scheduler:spawn_task(task_fn, priority) end

--- Starts or resumes the scheduler's execution loop.
--- The scheduler will continue to run tasks until all tasks are completed
--- or yield indefinitely.
function Scheduler:run() end

--- Executes a specified number of task steps.
--- If `count` is not provided, it executes a single step.
--- This method is useful for manual control of the scheduler's progression.
---@param count? integer The number of steps to execute. Defaults to 1.
function Scheduler:step(count) end

--- Checks if the scheduler has any tasks remaining.
---@return boolean
function Scheduler:has_tasks() end

--- Executes a specified number of task steps and then yields the coroutine.
--- This method is designed to be called from a coroutine, allowing the scheduler
--- to run for a limited duration before yielding control back to the caller.
---@param steps_before_yield? integer The number of steps to execute before yielding.
--- Defaults to 1. An error is raised for non-positive values.
function Scheduler:yielded_run(steps_before_yield)
    if steps_before_yield == nil then
        steps_before_yield = 1
    elseif type(steps_before_yield) ~= "number" then
        return error("Invalid type for a steps count")
    elseif steps_before_yield <= 0 then
        return error("Can't deal with non positives steps.")
    else
        while self:has_tasks() do
            self:step(steps_before_yield)
            coroutine.yield()
        end
    end
end

--- Returns a new FIFO (First-In, First-Out) scheduler instance.
--- Tasks are executed in the order they are added, in a round-robin fashion.
--- Any priority value provided to `spawn_task` will be ignored by this scheduler.
---@return Scheduler -- A new FIFO scheduler instance.
function scheduler.fifo() end

--- Returns a new Lottery scheduler instance.
--- Tasks are executed based on a probabilistic draw, where higher priority tasks
--- have a greater chance of being selected.
--- The `priority` value provided to `spawn_task` acts as the number of "tickets"
--- a task receives in the lottery draw.
---@return Scheduler -- A new Lottery scheduler instance.
function scheduler.lottery() end

scheduler = require("scheduler.core")

--- Injects the `yielded_run` method from the base `Scheduler` table into the shared
--- metatable index for all scheduler instances.
debug.getmetatable(scheduler.fifo()).__index.yielded_run = Scheduler.yielded_run
debug.getmetatable(scheduler.lottery()).__index.yielded_run = Scheduler.yielded_run

return scheduler
