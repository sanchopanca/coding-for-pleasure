with open('../input/p042_words.txt') as f:
    words = eval('[' + f.read() + ']')

numbers = {n * (n + 1) // 2 for n in range(1, 1000)}

result = 0
for word in words:
    score = sum([ord(c) - ord('A') + 1 for c in word])
    if score in numbers:
        result += 1

print(result)
