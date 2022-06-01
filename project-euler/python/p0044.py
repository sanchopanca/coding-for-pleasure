numbers = [n * (3 * n - 1) // 2 for n in range(1, 10000)]
s = set(numbers)

min_diff = float('inf')

for i, n in enumerate(numbers):
    for j in range(i+1, len(numbers)):
        m = numbers[j]
        if n + m in s and m - n in s:
            min_diff = min(min_diff, m - n)

print(min_diff)
