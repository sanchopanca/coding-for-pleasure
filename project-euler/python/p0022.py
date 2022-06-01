with open('../input/p022_names.txt') as f:
    names = eval('[' + f.read() + ']')
names.sort()

score = 0
for i, name in enumerate(names):
    score += sum([ord(c) - ord('A') + 1 for c in name]) * (i + 1)

print(score)
