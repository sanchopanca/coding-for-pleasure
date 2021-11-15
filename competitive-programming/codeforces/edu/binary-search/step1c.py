def bin_search_upper(a, x):
    l, r = -1, len(a)
    while l + 1 < r:
        m = (l + r) // 2
        if a[m] < x:
            l = m
        else:
            r = m
    return r


n, k = map(int, input().split())
array = list(map(int, input().split()))

for x in map(int, input().split()):
    print(bin_search_upper(array, x) + 1)
