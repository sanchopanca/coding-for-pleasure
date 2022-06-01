def sieve(n):
    s = [True] * (n + 1)
    for i in range(2, n + 1):
        if not s[i]:
            continue
        for j in range(i + i, n + 1, i):
            s[j] = False
    return {i for i in range(2, n + 1) if s[i]}


N = 1_000_000

primes = sieve(N)
sorted_primes = list(sorted(primes))

longest = 1
the_prime = 2

for i in range(len(sorted_primes)):
    candidate = sorted_primes[i]
    for j in range(i + 1, len(sorted_primes)):
        candidate += sorted_primes[j]
        if candidate not in primes:
            continue
        if j - i + 1 > longest:
            longest = j - i + 1
            the_prime = candidate

print(the_prime)

