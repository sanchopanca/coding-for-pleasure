def main():
    n, w = map(int, input().split())
    cheese = []
    for _ in range(n):
        a, b = map(int, input().split())
        cheese.append((a, b))
    cheese.sort(reverse=True)
    i = 0
    d = 0
    while w > 0 and i < n:
        g = min(w, cheese[i][1])
        w -= g
        d += g * cheese[i][0]
        i += 1
    print(d)


main()
