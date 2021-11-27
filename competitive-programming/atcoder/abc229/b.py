def main():
    a, b = map(int, input().split())
    a = str(a)
    b = str(b)
    l = max(len(a), len(b))
    a = a.zfill(l)
    b = b.zfill(l)
    for i in range(l):
        if int(a[i]) + int(b[i]) > 9:
            print('Hard')
            return
    print('Easy')


main()
