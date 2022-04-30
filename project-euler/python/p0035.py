def sieve(n):
    s = [True] * (n + 1)
    for i in range(2, n + 1):
        if not s[i]:
            continue
        for j in range(i + i, n + 1, i):
            s[j] = False
    return [i for i in range(2, n + 1) if s[i]]

primes = set(sieve(1_000_000))

res = 0
for n in primes:
    s = str(n)
    if len(s) == 1:
        res += 1
        continue
    for i in range(1, len(s)):
        if int(s[i:] + s[:i]) not in primes:
            break
    else:
        res += 1
print(res)
