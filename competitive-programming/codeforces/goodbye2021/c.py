from copy import copy
from decimal import Decimal


def solve1(a, n, step):
    x = 0
    for i in range(1, n):
        if a[i] != a[i - 1] + step:
            a[i] = a[i - 1] + step
            x += 1
    return x

def solve2(a, n, step):
    x = 0
    if a[0] != a[1] - step:
        a[0] = a[1] - step
        x += 1
    for i in range(2, n):
        if a[i] != a[i - 1] + step:
            a[i] = a[i - 1] + step
            x += 1
    return x

def solve(a, n):
    answer = n - 1
    step_whole = range(0, 101)
    # step_part = (0, Decimal('0.5'), Decimal('0.25'), Decimal('0.75'), Decimal('0.125'), Decimal('0.375'), Decimal('0.625'), Decimal('0.2'), Decimal('0.4'), Decimal('0.8'))
    # step_part = (0, Decimal('0.5'), Decimal('0.25'), Decimal('0.75'))
    step_part = (0, Decimal('0.5'))
    for i in step_whole:
        for j in step_part:
            answer = min(answer, solve1(copy(a), n, i + j))
            answer = min(answer, solve2(copy(a), n, i + j))
    return answer


def main():
    t = int(input())
    for _ in range(t):
        n = int(input())
        a = list(map(int, input().split()))
        if n == 1 or n == 2:
            print(0)
            continue
        print(solve(a, n))


main()
