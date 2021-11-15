def bin_search_lower(a, x):
    l, r = -1, len(a)
    while l + 1 < r:
        m = (l + r) // 2
        if a[m] <= x:
            l = m
        else:
            r = m
    return l


n, k = map(int, input().split())
array = list(map(int, input().split()))

for x in map(int, input().split()):
    print(bin_search_lower(array, x) + 1)
