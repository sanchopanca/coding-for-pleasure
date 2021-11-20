def main():
    t = int(input())
    # t = 1
    for _ in range(t):
        n, m = sorted(map(int, input().split()))
        # n, m = 5, 7
        if n % 3 == 0 or m % 3 == 0:
            print(n * m // 3)
            continue
        if n % 3 == 1 and m % 3 == 1:
            print((m // 3) * n + n // 3 + 1)
            continue
        if n % 3 == 2 and m % 3 == 2:
            print((m // 3) * n + 2 * (n // 3 + 1))
            continue
        if n % 3 == 2:
            print((m // 3) * n + n // 3 + 1)
            continue
        print((m // 3) * n + 2 * (n // 3 + 1) - 1)



main()
