use mlua::prelude::*;

/// The core implementation of the vector, containing the business logic.
///
/// This struct holds the actual data (`Vec<LuaValue>`) and performs all vector
/// operations. It is designed to be independent of the `mlua` binding layer,
/// allowing for reusable logic. The `PartialEq` derive implements a deep
/// comparison for the `__eq` metamethod.
#[derive(PartialEq)]
pub(crate) struct LuaVector {
    /// The internal `Vec` that stores the Lua values.
    inner: Vec<LuaValue>,
}

impl LuaVector {
    /// The maximum length of the vector, constrained by the Lua `integer` type.
    const MAX_LEN: usize = LuaInteger::MAX as usize;

    /// Creates a new `LuaVector` instance with a given size and filler value.
    ///
    /// Returns an `Err` if the size is a negative number.
    pub(crate) fn new(size: LuaInteger, filler: LuaValue) -> Result<Self, String> {
        if size < 0 {
            return Err(format!("Invalid Vector len {}.", size));
        }
        Ok(Self {
            inner: vec![filler; size as usize],
        })
    }

    /// Creates a new `LuaVector` from a pre-existing Rust `Vec<LuaValue>`.
    pub(crate) fn of(values: Vec<LuaValue>) -> Self {
        Self { inner: values }
    }

    /// Retrieves a value from the vector based on a Lua 1-based index.
    ///
    /// This method supports positive and negative indexing. A positive index `i`
    /// retrieves the element at that position, while a negative index `-j` retrieves
    /// the element `j` from the end of the vector.
    pub(crate) fn get(&self, index: LuaInteger) -> Option<LuaValue> {
        self.zero_based_index(index).map(|idx| self.inner[idx].clone())
    }

    /// Returns the number of elements in the vector.
    pub(crate) fn len(&self) -> LuaInteger {
        self.inner.len() as LuaInteger
    }

    /// Appends a new value to the end of the vector.
    ///
    /// Returns `true` if the push was successful, and `false` if the vector
    /// has reached its maximum capacity.
    pub(crate) fn push(&mut self, value: LuaValue) -> bool {
        if self.inner.len() < Self::MAX_LEN {
            self.inner.push(value);
            return true;
        }
        false
    }

    /// Removes and returns the last element of the vector.
    ///
    /// Returns `None` if the vector is empty.
    pub(crate) fn pop(&mut self) -> Option<LuaValue> {
        self.inner.pop()
    }

    /// Sets the value at a given Lua index.
    ///
    /// Returns `true` on success, `false` if the index is out of bounds.
    /// This method supports both positive and negative indexing.
    pub(crate) fn set(&mut self, index: LuaInteger, new_value: LuaValue) -> bool {
        match self.zero_based_index(index) {
            Some(idx) => {
                self.inner[idx] = new_value;
                true
            }
            None => false,
        }
    }

    /// Compares two `LuaVector` instances for pointer equality.
    ///
    /// This method is a helper for the `is_same` Lua method, which performs a reference
    /// comparison.
    pub(crate) fn is_same(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }

    /// A helper function to convert a Lua 1-based index to a Rust 0-based index.
    ///
    /// This function also handles negative indices for reverse indexing.
    /// - Positive `idx` -> `idx - 1`
    /// - Negative `idx` -> `idx + len`
    /// - Returns `None` if the index is out of bounds.
    fn zero_based_index(&self, one_based_index: LuaInteger) -> Option<usize> {
        let len = self.inner.len() as LuaInteger;

        match one_based_index {
            0 => None,
            idx if idx > len => None,
            idx if idx < -len => None,
            idx if idx > 0 => Some((idx - 1) as usize),
            idx => Some((idx + len) as usize),
        }
    }
}