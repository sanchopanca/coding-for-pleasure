def bin_search_lower(a, x):
    l, r = -1, len(a)
    while l + 1 < r:
        m = (l + r) // 2
        if a[m] <= x:
            l = m
        else:
            r = m
    return l


def bin_search(a, x):
    i = bin_search_lower(a, x)
    return i != -1 and a[i] == x


n, k = map(int, input().split())
array = list(map(int, input().split()))

for x in map(int, input().split()):
    print("YES" if bin_search(array, x) else "NO")
