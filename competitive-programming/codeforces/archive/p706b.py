def main():
    n = int(input())
    x = sorted(map(int, input().split()))
    q = int(input())
    m = [int(input()) for _ in range(q)]
    for coin in m:
        l, r = -1, n
        while l + 1 < r:
            mid = (l + r) // 2
            if x[mid] <= coin:
                l = mid
            else:
                r = mid
        print(l + 1)


main()
