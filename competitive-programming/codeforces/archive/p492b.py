def main():
        n, l = map(int, input().split())
        a = sorted(map(int, input().split()))
        max_gap = 0 if len(a) < 2 else max(b - a for a, b in zip(a, a[1:]))
        d = max(a[0], l - a[-1], max_gap / 2)
        print(d)


main()
