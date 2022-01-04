def main():
    n, k = map(int, input().split())
    time = 240 - k
    l, r = 0, 100
    while l + 1 < r:
        m = (l + r) // 2
        if 5 * m * (1 + m) // 2 <= time:
            l = m
        else:
            r = m
    print(min(l, n))


main()
