def f(ropes, length, needed):
    x = 0
    for r in ropes:
        x += r // length
    return x >= needed


def main():
    n, k = map(int, input().split())
    ropes = []
    for _ in range(n):
        ropes.append(float(input()))
    l, r = 0, 1e18
    for _ in range(100):
        m = (l + r) / 2
        if f(ropes, m, k):
            l = m
        else:
            r = m
    print(l)


main()
