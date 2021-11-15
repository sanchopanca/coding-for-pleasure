def f(x, n, w, h):
    return (x // h) * (x // w) >= n


def main():
    w, h, n = map(int, input().split())
    l, r = 0, 10 ** 18
    while l + 1 < r:
        m = (l + r) // 2
        if f(m, n, w, h):
            r = m
        else:
            l = m
    print(r)

main()
