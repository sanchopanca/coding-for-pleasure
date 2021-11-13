def main():
    n, k, a = map(int, input().split())
    a -= 1
    # a = 0
    res = (a + k) % n
    if res == 0:
        res = n
    print(res)


main()
