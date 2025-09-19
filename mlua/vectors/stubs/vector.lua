---@class Vector A Lua object that provides a dynamic array with common vector operations.
local Vector = {}

--- Creates a new, empty `Vector` instance.
---@return Vector
function Vector.new() end

--- Pushes a value to the end of the vector.
---@generic T
---@param self Vector<T>
---@param value T The value to push.
---@return boolean Returns `true` if the push was successful.
function Vector.push(self, value) end

--- Removes the last value from the vector.
---@generic T
---@param self Vector<T>
---@return boolean, T Returns `true` and the popped value on success, or `false, nil` if the vector is empty.
function Vector.pop(self) end

--- Retrieves a value from the vector by its index.
---@generic T
---@param self Vector<T>
---@param index integer The index of the value.
---@return T|nil Returns the value at the given index, or `nil` if the index is out of bounds.
function Vector.get(self, index) end

--- Sets a new value at a specific index.
---
--- This method will cause an error if the index is out of bounds.
---@generic T
---@param self Vector<T>
---@param index integer The index to set the new value.
---@param value T The new value.
function Vector.set(self, index, value) end

--- Returns the current number of elements in the vector.
---@generic T
---@param self Vector<T>
---@return integer
function Vector.len(self) end

return require("vector.core")