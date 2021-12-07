with open('../input/07.txt') as f:
    numbers = [int(x) for x in f.read().split(',')]


def solve(cost_func):
    min_cost = float('inf')
    for i in range(min(numbers), max(numbers) + 1):
        c = cost_func(numbers, i)
        if c < min_cost:
            min_cost = c
    print(min_cost)


def cost1(numbers, position):
    c = 0
    for n in numbers:
        c += abs(n - position)
    return c


def cost2(numbers, position):
    c = 0
    for n in numbers:
        x = abs(n - position)
        c += x * (x + 1) // 2
    return c


solve(cost1)
solve(cost2)
