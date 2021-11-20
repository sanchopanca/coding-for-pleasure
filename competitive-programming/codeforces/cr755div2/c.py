def main():
    t = int(input())
    for _ in range(t):
        n = int(input())
        a = sorted(map(int, input().split()))
        b = sorted(map(int, input().split()))
        for i in range(n):
            if b[i] - a[i] > 1:
                print('NO')
                break
        else:
            print('YES')

main()
