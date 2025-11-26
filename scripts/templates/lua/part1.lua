local startTime = os.clock()

local function read_input(filePath)
    local f = io.open(filePath, "r")
    if not f then return {} end
    local lines = {}
    for line in f:lines() do
        if #line > 0 then
            table.insert(lines, line)
        end
    end
    f:close()
    return lines
end

local function solve(input)
    -- Your solution for part 1 goes here
    return "solution for part 1"
end

local inputFile = arg[1] or "input.txt"
local input = read_input(inputFile)
local solution = solve(input)

local endTime = os.clock()
local duration = endTime - startTime

print("Solution: " .. solution)
print(string.format("Completed in %.4f seconds", duration))
