def bin_search_upper(a, x):
    l, r = -1, len(a)
    while l + 1 < r:
        m = (l + r) // 2
        if a[m] <= x:
            l = m
        else:
            r = m
    return r


def bin_search_lower(a, x):
    l, r = -1, len(a)
    while l + 1 < r:
        m = (l + r) // 2
        if a[m] < x:
            l = m
        else:
            r = m
    return l


n = int(input())
array = list(sorted(map(int, input().split())))
k = int(input())

for _ in range(k):
    l, r = map(int, input().split())
    print(bin_search_upper(array, r) - bin_search_lower(array, l) - 1, end=' ')
print()

