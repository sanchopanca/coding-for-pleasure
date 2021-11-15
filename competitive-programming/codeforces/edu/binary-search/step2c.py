def can_copy(n, time, x, y):
    done = 1
    time -= min(x, y)
    if time < 0:
        return False
    done += time // x + time // y
    return done >= n


def main():
    n, x, y = map(int, input().split())
    l, r = 0, 10**18
    while l + 1 < r:
        m = (l + r) // 2
        if can_copy(n, m, x, y):
            r = m
        else:
            l = m
    print(r)

main()
