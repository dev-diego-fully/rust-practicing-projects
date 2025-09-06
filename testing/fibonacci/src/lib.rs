//! A module for generating the Fibonacci sequence.
//!
//! This module provides an `Iterator` over the Fibonacci sequence and helper
//! functions to retrieve specific numbers from it.
//!
//! The sequence starts with `0, 1, 1, 2, 3, 5, ...`
//!
//! # Examples
//!
//! ```
//! let mut fib = fibonacci::serie();
//! assert_eq!(fib.next(), Some(0));
//! assert_eq!(fib.next(), Some(1));
//! assert_eq!(fib.next(), Some(1));
//! assert_eq!(fib.next(), Some(2));
//! ```

/// Returns a new iterator over the Fibonacci sequence.
///
/// This is a convenient factory function for creating `FibonacciSerie` instances.
pub fn serie() -> impl Iterator<Item = i64> {
    FibonacciSerie::new()
}

/// Returns the nth Fibonacci number in the sequence.
///
/// This function creates a new iterator and advances it `n` times to find the
/// desired number. Returns `None` if the sequence ends prematurely due to overflow.
///
/// # Arguments
///
/// * `n` - The index of the Fibonacci number to retrieve.
///
/// # Examples
///
/// ```
/// assert_eq!(fibonacci::nth(5), Some(5));
/// assert_eq!(fibonacci::nth(10), Some(55));
/// assert!(matches!(fibonacci::nth(92), Some(_)));
/// assert_eq!(fibonacci::nth(93), None);
/// ```
pub fn nth(n: usize) -> Option<i64> {
    FibonacciSerie::new().nth(n)
}

/// A struct that generates the Fibonacci sequence as an iterator.
///
/// It holds the two previous numbers in the sequence to calculate the next one.
struct FibonacciSerie {
    /// The last number generated in the sequence.
    last_one: Option<i64>,
    /// The number generated before `last_one`.
    last_two: Option<i64>,
}

impl FibonacciSerie {
    /// Creates a new `FibonacciSerie` instance, initialized to the start of the sequence.
    fn new() -> Self {
        Self {
            last_one: None,
            last_two: None,
        }
    }
}

impl Iterator for FibonacciSerie {
    type Item = i64;

    /// Advances the iterator to the next number in the Fibonacci sequence.
    ///
    /// This method performs a pattern match to handle the initial state and the
    /// main recursive step. It uses `checked_add` to safely handle potential
    /// integer overflow. The iterator returns `None` when a number in the sequence
    /// cannot be represented by the `i64` type.
    fn next(&mut self) -> Option<Self::Item> {
        (self.last_one, self.last_two) = match (self.last_one, self.last_two) {
            (None, None) => (Some(0), None),
            (Some(_), None) => (Some(1), Some(0)),
            (Some(lo), Some(lt)) => (lo.checked_add(lt), Some(lo)),
            (None, Some(_)) => return None
        };
        self.last_one
    }
}