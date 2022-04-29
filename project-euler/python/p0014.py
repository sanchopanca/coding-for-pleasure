from functools import cache


@cache
def collatz(n):
    if n == 1:
        return 1
    if n % 2:
        return 1 + collatz(3 * n + 1)
    else:
        return 1 + collatz(n // 2)

res, m = 0, 0
for n in range (1, 1_000_000):
    l = collatz(n)
    if l > res:
        res, m = l, n

print(m)
