t = int(input())

for _ in range(t):
    n = int(input())
    a = sorted(map(int, input().split()))
    to_eliminate = a[0]
    maximum = a[0]
    for i in range(1, len(a)):
        maximum = max(maximum, a[i] - to_eliminate)
        to_eliminate += a[i] - to_eliminate
    print(maximum)
