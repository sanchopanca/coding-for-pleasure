n = int(input())
while n != -1:
    log = [(0, 0)]
    for i in range(n):
        s, t = map(int, input().split())
        log.append((s, t))
    d = 0
    for (_, t1), (s, t2) in zip(log[:-1], log[1:]):
        d += s * (t2 - t1)
    print(d, 'miles')
    n = int(input())
