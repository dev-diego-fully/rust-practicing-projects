---@diagnostic disable: undefined-field

local vector = require("vector")

describe("Uses one-based indexes", function()
    describe("This means that the first index is 1", function()
        it("In the \"get\" method", function()
            local expected = 0;

            local vec = vector.new();
            vec:push(expected);

            local _, value = vec:get(1);
            assert.are.equals(expected, value);
        end);

        it("In the \"set\" method", function()
            local initial = 0;
            local expected = "first";

            local vec = vector.new();
            vec:push(initial);
            vec:set(1, expected);

            local _, value = vec:get(expected)

            assert.are.equals(expected, value);
        end);

        it("In the \"__index\" meta method", function()
            local expected = 0;

            local vec = vector.new();
            vec:push(expected);

            assert.has.no_errors(function()
                assert.are.equals(expected, vec[1]);
            end);
        end);

        it("In the \"__newindex\" meta method", function()
            local initial = 0;
            local expected = "first";

            local vec = vector.new();
            vec:push(initial);
            vec[1] = expected;

            assert.has.no_errors(function()
                assert.are.equals(expected, vec[1]);
            end);
        end);
    end);

    describe("This means that the last index is len", function()
        it("In the \"get\" method", function()
            local expected = "value";
            local vec = vector.of(nil, nil, nil, expected);
            local is_present, value = vec:get(#vec);

            assert.is.truthy(is_present);
            assert.are.equals(expected, value);
        end);

        it("In the \"set\" method", function()
            local initial = "initital"
            local expected = "expected";

            local vec = vector.of(nil, nil, nil, initial);
            local is_present = vec:set(#vec, expected);

            assert.is.truthy(is_present);
            assert.are.equals(expected, vec[#vec]);
        end);

        it("In the \"__index\" meta method", function()
            local expected = "expected";

            local vec = vector.of(nil, nil, nil, expected);

            assert.are.equals(expected, vec[#vec]);
        end);

        it("In the \"__newindex\" meta method", function()
            local initial = "initial";
            local expected = "expected";

            local vec = vector.of(nil, nil, nil, initial);
            vec[#vec] = expected;

            assert.are.equals(expected, vec[#vec]);
        end);
    end);
end);

describe("new", function()
    it("Creates an empty vector", function()
        local vec = vector.new();
        assert.are.equals(0, vec:len());
    end);

    it("Creates an array with the given size but padded with nils", function()
        local expected_len = 10;
        local vec = vector.new(expected_len);

        assert.are.equals(expected_len, vec:len());

        for i = 1, expected_len, 1 do
            assert.are.equals(nil, vec[i]);
        end
    end);

    it("Creates an array with the given size and filled with the given value", function()
        local expected_len = 10;
        local vec = vector.new(expected_len, true);

        assert.are.equals(expected_len);

        for i = 1, expected_len, 1 do
            assert.are.equals(true, vec[i]);
        end
    end);
end);

describe("of", function()
    it("Creates a vector with the given elements", function()
        local expecteds = { 0, 1, 2, 3 };
        local vec = vector.of(0, 1, 2, 3);

        assert.are.equals(4, vec:len());

        for i, _ in ipairs(expecteds) do
            assert.are.equals(expecteds[i], vec[i]);
        end
    end);
end)

describe("get", function()
    it("Returns a present element along with a presence indicator", function()
        local expected_value = 10;

        local vec = vector.new();
        vec:push(expected_value);

        local is_present, value = vec:get(1);

        assert.is.truthy(is_present);
        assert.are.equals(expected_value, value);
    end);

    it("There may be nils There may be \"nils\" as a present value", function()
        local vec = vector.new();
        vec:push(nil);

        local is_present, value = vec:get(1);

        assert.is.truthy(is_present);
        assert.are.equals(nil, value);
    end);

    it("An index outside the bounds is not considered present and its value is treated as nil.", function()
        local out_of_bound_index = 2

        local vec = vector.new();
        vec:push(10);

        local is_present, value = vec:get(out_of_bound_index);

        assert.is.falsy(is_present);
        assert.are.equals(nil, value);
    end);
end);

describe("set", function()
    it("Returns true if used on an index within the bounds of the Vector", function()
        local len = 5;
        local tested_index = 2;

        local vec = vector.new(len);
        local succeed = vec:set(tested_index);

        assert.is.truthy(succeed);
    end);

    it("Returns false if used on an index outside the bounds of the Vector", function()
        local len = 5;
        local tested_index = len + 1;

        local vec = vector.new(len);
        local succeed = vec:set(tested_index);

        assert.is.falsy(succeed);
    end);
end);

describe("push", function()
    describe("Increase the size of the current vector", function()
        local starting_len = 5;
        local expected_len = starting_len + 1;

        local vec = vector.new(starting_len);
        vec:push(nil);

        assert.are.equals(expected_len, #vec);
    end);

    describe("Adds a new element at the end of the Vector", function()
        local initial_len = 10
        local expected = "expected";

        local vec = vector.new(initial_len);
        vec:push(expected);

        assert.are.equals(expected, vec[#vec]);
    end);
end)

describe("pop", function()
    it("On an empty Vector it returns a failure flag and a nil value", function()
        local vec = vector.new();

        local is_present, value = vec:pop();

        assert.is.falsy(is_present);
        assert.are.equals(value);
    end);

    it("On a non-empty vector, returns a success and the last element of the vector", function()
        local expected_value = true;

        local vec = vector.of(nil, nil, nil, expected_value);
        local is_present, value = vec:pop();

        assert.is.truthy(is_present);
        assert.are.equals(expected_value, value);
    end);

    it("Removes and returns the last element of the Vector", function()
        local expected_value = "expected";
        local expected_len = 3;

        local vec = vector.of(true, false, nil, expected_value);

        local _, value = vec:pop();

        assert.are.equals(expected_len, #vec);
        assert.are.equals(expected_value, value);
    end);
end);

describe("is_same", function()
    it("True when used in the same vector", function()
        local vec = vector.of(1, 2, 3, 4);

        assert.is.truthy(vec:is_same(vec));
    end);

    it("False between distinct but equal vectors", function()
        local vec1 = vector.of(1, 2, 3, 4);
        local vec2 = vector.of(1, 2, 3, 4);

        assert.is.falsy(vec1:is_same(vec2));
    end);

    it("False when used on different vectors", function()
        local vec1 = vector.of(1, 2, 3, 4);
        local vec2 = vector.of(2, 3, 1, 4);

        assert.is.falsy(vec1:is_same(vec2));
    end);
end)

describe("__index", function()
    it("Causes error if used on an index outside the Vector's bounds", function()
        local vec = vector.of(4, 6, 5, 7);

        assert.has.errors(function()
            local _ = vec[#vec];
        end);
    end);

    it("Retrieves the value without using presence flags", function()
        local expected = 4;
        local vec = vector.of(expected);
        local value = vec[1];

        assert.are.equals(expected, value);
    end);
end);

describe("__newindex", function()
    it("Causes error if used on an index outside the Vector's bounds", function()
        local vec = vector.of(4, 6, 5, 7);

        assert.has.errors(function()
            vec[#vec + 1] = 10;
        end);
    end);

    it("Modifies the value at the given index if it is within the bounds", function()
        local len = 10;
        local expected = true;
        local vec = vector.new(len, false);

        assert.has.no_errors(function()
            vec[len] = expected;
        end);

        assert.are.equals(expected, vec[len]);
    end);
end);

describe("__len", function()
    it("This is correct for empty vectors", function()
        local expected_len = 0;
        local vec = vector.new();

        assert.are.equals(expected_len, #vec);
    end);

    it("It is correct for a Vector created with elements", function()
        local expected_len = 5;
        local vec = vector.new(expected_len, nil);

        assert.are.equals(expected_len, #vec);
    end);

    it("Increases with each new insertion", function()
        local max_tested_len = 100;
        local vec = vector.new();

        for expected_len = 1, max_tested_len, 1 do
            assert.are.equals(expected_len, #vec);
        end
    end);

    it("Decreases with each removal", function()
        local max_tested_len = 100;
        local vec = vector.new(max_tested_len);

        for expected_len = max_tested_len, 0, 1 do
            assert.are.equals(expected_len, #vec);
        end
    end);
end)

describe("__eq", function()
    it("Vectors are equal to themselves.", function()
        local vec = vector.of(1, 2, 3, 4);

        assert.are.equals(vec, vec);
    end);

    it("Vectors with equal elements and in the same order are equal", function()
        local expected = vector.of(4, 3, 2, 1);
        local vec = vector.of(4, 3, 2, 1);

        assert.are.equals(expected, vec);
    end);

    it("Vectors with the same elements but in different order are not equal", function()
        local expected = vector.of(4, 3, 2, 1);
        local vec = vector.of(1, 2, 3, 4);

        assert.are.equals(expected, vec);
    end);
end)
