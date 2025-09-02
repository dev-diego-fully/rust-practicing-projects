//! This module provides a `Lottery` task list, which selects the next task
//! to be executed based on a weighted random "lottery."
//!
//! Tasks with a higher priority have a proportionally higher chance of being
//! selected, mimicking a lottery system where each unit of priority is a
//! ticket.
//!
use super::TaskList;
use crate::tasks::Task;
use rand::{prelude::*, rng};

/// A task list that picks the next task using a weighted random selection.
///
/// The probability of a task being chosen is proportional to its priority.
pub(crate) struct Lottery {
    /// The collection of tasks to be scheduled.
    tasks: Vec<Task>,
    /// The random number generator used for the lottery.
    randomizer: ThreadRng,
}

impl TaskList for Lottery {
    type That = Self;

    /// Creates a new, empty `Lottery` task list.
    fn new() -> Self::That {
        Self {
            tasks: Vec::new(),
            randomizer: rng(),
        }
    }

    /// Selects and removes a task from the list based on a weighted random choice.
    ///
    /// The task's priority determines its likelihood of being chosen. The
    /// function returns `None` if the list is empty.
    fn peek(&mut self) -> Option<Task> {
        if self.is_empty() {
            return None;
        }
        let index = self.choose();
        Some(self.tasks.remove(index))
    }

    /// Adds a new task to the list.
    ///
    /// The task's priority will be used for weighted selection.
    fn add(&mut self, task: Task) {
        self.tasks.push(task);
    }

    /// Checks if the task list is empty.
    fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }
}

impl Lottery {
    /// Chooses the index of a task based on its priority-weighted tickets.
    ///
    /// This private method performs the core lottery logic.
    fn choose(&mut self) -> usize {
        let ticket = self.randomizer.random_range(0..self.total_tickets());
        self.index_of_ticket(ticket)
    }

    /// Calculates the total number of tickets (sum of all task priorities).
    fn total_tickets(&self) -> usize {
        self.tasks.iter().map(|task| task.priority() as usize).sum()
    }

    /// Finds the index of the task that corresponds to the winning ticket.
    ///
    /// It iterates through the tasks, accumulating tickets until the winning
    /// ticket number is reached.
    fn index_of_ticket(&self, ticket: usize) -> usize {
        let mut total_tickets = 0;

        for (idx, task) in self.tasks.iter().enumerate() {
            total_tickets += task.priority() as usize;

            if total_tickets >= ticket {
                return idx;
            }
        }

        0
    }
}