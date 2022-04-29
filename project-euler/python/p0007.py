def sieve(n):
    s = [True] * (n + 1)
    for i in range(2, n + 1):
        if not s[i]:
            continue
        for j in range(i + i, n + 1, i):
            s[j] = False
    return [i for i in range(2, n + 1) if s[i]]

primes = []
n = 1
while len(primes) < 10_001:
    n *= 2
    primes = sieve(n)

print(primes[10_000])
