from math import prod

s = ''.join(str(x) for x in range(1, 1_000_000))

print(prod(map(int, [s[10 ** i - 1] for i in range(7)])))
