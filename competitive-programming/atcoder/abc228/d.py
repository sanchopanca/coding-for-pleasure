def main():
    N = 1048576
    q = int(input())
    a = [-1] * N
    for _ in range(q):
        t, x = map(int, input().split())
        if t == 1:
            h = x
            while a[h % N] != -1:
                h += 1
            a[h % N] = x
        else:
            print(a[x % N])

main()
