file = io.open("../input/05.txt", "r")
memory = {}
memory2 = {}
for line in file:lines() do
    table.insert(memory, tonumber(line))
    table.insert(memory2, tonumber(line))
end

i = 1
res1 = 0
while i > 0 and i <= #memory do
    local offset = memory[i]
    memory[i] = memory[i] + 1
    i = i + offset
    res1 = res1 + 1
end

i = 1
res2 = 0
while i > 0 and i <= #memory2 do
    local offset = memory2[i]
    if offset >= 3 then
        memory2[i] = memory2[i] - 1
    else
        memory2[i] = memory2[i] + 1
    end
    i = i + offset
    res2 = res2 + 1
end

print(res1)
print(res2)
