def main():
    n = int(input())
    a = list(map(int, input().split()))
    ranges = []
    cur = 1
    for x in a:
        ranges.append((cur, cur + x))
        cur += x

    # print(ranges)
    m = int(input())
    for q in map(int, input().split()):
        l, r = -1, n
        while l + 1 < r:
            mid = (l + r) // 2
            if ranges[mid][0] <= q < ranges[mid][1]:
                print(mid + 1)
                break
            elif q < ranges[mid][0]:
                r = mid
            else:
                l = mid
        else:
            if r != n and ranges[r][0] <= q < ranges[r][1]:
                print(r + 1)
            else:
                print(l + 1)


main()
