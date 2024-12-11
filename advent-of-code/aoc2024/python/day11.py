from functools import cache


@cache
def solve(n, steps):
    s = str(n)
    l = len(s)
    if steps == 0:
        return 1
    elif n == 0:
        return solve(1, steps - 1)
    elif l % 2 == 0:
        return solve(int(s[: l // 2]), steps - 1) + solve(int(s[l // 2 :]), steps - 1)
    else:
        return solve(n * 2024, steps - 1)


if __name__ == "__main__":
    with open("../input/11.txt") as f:
        numbers = list(map(int, f.readline().split(" ")))
        print(numbers)
        print(sum(solve(n, 25) for n in numbers))
        print(sum(solve(n, 75) for n in numbers))
