def d(n):
    s = 1
    i = 2
    while i * i <= n:
        if n % i == 0:
            s += i
            s += n // i
        i += 1
    return s


m = {}
for i in range(2, 10_001):
    m[i] = d(i)

res = 0
for n, s in m.items():
    if m.get(s, -1) == n and s != n:
        res += n

print(res)
