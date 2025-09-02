//! This module defines the `TaskList` trait and provides different implementations
//! for managing a queue of tasks.
//!
mod fifo;
mod lottery;

use crate::tasks::Task;

pub(crate) use fifo::FIFOTaskList;
pub(crate) use lottery::Lottery;

/// A trait that defines the common interface for a task list.
///
/// This trait allows different scheduling policies (e.g., FIFO, Lottery)
/// to be used interchangeably by the scheduler.
pub(crate) trait TaskList {

    /// The concrete type that implements this trait.
    type That;

    /// Creates a new instance of the task list.
    fn new() -> Self::That;

    /// Removes and returns the next task to be executed.
    ///
    /// The specific task returned depends on the implementation's scheduling
    /// policy. Returns `None` if the list is empty.
    fn peek(&mut self) -> Option<Task>;

    /// Adds a new task to the list.
    fn add(&mut self, task: Task);

    /// Checks if the task list is empty.
    fn is_empty(&self) -> bool;

}