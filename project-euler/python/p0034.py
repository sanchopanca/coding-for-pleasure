from math import prod


def digit_factorial(n):
    digits = map(int, str(n))
    factorials = [prod(range(1, x+1)) for x in digits]
    return sum(factorials)


s = 0
for i in range(10, 1_000_000):
    if digit_factorial(i) == i:
        s += i
print(s)
