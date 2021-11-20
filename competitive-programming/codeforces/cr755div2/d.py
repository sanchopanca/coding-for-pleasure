def bin_search_lower(n, x):
    l, r = 0, n+1
    while l + 1 < r:
        m = (l + r) // 2
        if make_guess(l, r) == x:
            l = m
        else:
            r = m
    return l

def bin_search_upper(n, x):
    l, r = 0, n
    while l + 1 < r:
        m = (l + r) // 2
        if make_guess(l, r) == x:
            r = m
        else:
            l = m
    return r

def bin_search(a, x):
    i = bin_search_lower(a, x)
    return i != -1 and a[i] == x

def make_guess(l, r):
    print('?', l, r, flush=True)
    x = int(input())
    return x

def guess_permutation(n):
    l, r = 1, n
    x = make_guess(l, r)
    solid_l, solid_r = l, r
    while True:
        l = (r - l) // 2
        new_x = make_guess(l, r)
        if new_x == x:
            solid_l = l



def main():
    t = int(input())
    for _ in range(t):
        n = int(input())
        guess_permutation(n)


main()
