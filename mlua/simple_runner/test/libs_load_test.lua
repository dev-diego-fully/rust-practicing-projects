---
---@param test_name string
---@param value any
---@param on_success_message string
---@param on_fail_message string
local function test(test_name, value, on_success_message, on_fail_message)
    if value then
        print("Ok: " .. test_name .. " -> " .. on_success_message)
    else
        print("Failed" .. test_name .. " -> " .. on_fail_message)
    end
end

test("Correctly loaded print", print ~= nil, "loaded basic modules", "not loaded basic modules")
test("Correctly loaded math", math ~= nil, "loaded lua safe std", "not loaded safe lua std")
test("Correctly not loaded debug", debug == nil, "loaded lua unsafe std", "not load lua unsafe std")
