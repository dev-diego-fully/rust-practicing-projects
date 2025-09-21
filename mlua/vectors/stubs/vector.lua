---@diagnostic disable: missing-return

local vector = {};

---@class Vector A Lua object that provides a dynamic array with common vector operations.
local Vector = {}

--- Creates a new vector.
---
--- The constructor has several behaviors based on the arguments provided:
--- - `vector.new()`: Creates an empty vector.
--- - `vector.new(initial_size)`: Creates a vector of the given size, with all elements initialized to `nil`.
--- - `vector.new(initial_size, filler_value)`: Creates a vector of the given size, with all elements filled with `filler_value`.
---@generic T
---@param initial_size? integer The initial size of the vector.
---@param filler_value? T The value used to fill the vector.
---@return Vector<T>
function vector.new(initial_size, filler_value)
end

--- Creates a new vector from a list of elements.
---@generic T
---@param ... T The elements to be added to the vector.
---@return Vector<T>
function vector.of(...)

end

--- Retrieves an element from the vector by its index.
---
--- This method returns two values: a presence flag and the element's value.
--- This allows for distinguishing between a `nil` value that is present and a value that is absent (out of bounds).
---@generic T
---@param self Vector<T>
---@param index integer The index of the element to retrieve.
---@return boolean, T | nil -- A boolean flag indicating presence and the element's value.
function Vector.get(self, index)
end

--- Sets a new value at a specific index.
---
--- This method returns a boolean indicating whether the operation succeeded. It will
--- succeed only if the index is within the vector's current bounds.
---@generic T
---@param self Vector<T>
---@param index integer The index to set the value.
---@param value T The new value.
---@return boolean -- Returns `true` if the value was set, `false` otherwise.
function Vector.set(self, index, value)
end

--- Adds a new element to the end of the vector.
---@generic T
---@param self Vector<T>
---@param value T The value to be pushed.
---@return boolean -- Returns `true` if the operation was successful.
function Vector.push(self, value)
end

--- Removes and returns the last element from the vector.
---
--- This method returns two values: a success flag and the element's value.
--- It returns `true` and the element on success, or `false` and `nil` if the vector is empty.
---@generic T
---@param self Vector<T>
---@return boolean, T | nil -- The success flag and the popped value.
function Vector.pop(self)
end

--- Compares two vectors for reference equality.
---
--- This method returns `true` only if the two objects are the exact same instance in memory.
--- It is distinct from the `==` operator, which performs a value-based comparison.
---@generic T
---@param self Vector<T>
---@param other any The other value to compare against.
---@return boolean Returns `true` if `self` and `other` refer to the same object, `false` otherwise.
function Vector.is_same(self, other)
end

vector = require("vector.core")

return vector