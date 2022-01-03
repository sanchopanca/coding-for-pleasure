def main():
    t = int(input())
    for _ in range(t):
        n = int(input())
        a = map(int, input().split())
        s = set()
        for nmbr in a:
            if nmbr in s:
                s.add(-nmbr)
            else:
                s.add(nmbr)
        print(len(s))


main()
