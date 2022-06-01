def sieve(n):
    s = [True] * (n + 1)
    for i in range(2, n + 1):
        if not s[i]:
            continue
        for j in range(i + i, n + 1, i):
            s[j] = False
    return {i for i in range(2, n + 1) if s[i]}


primes = sieve(100_000)
max_primes = 0
coefficients = 0


for a in range(-999, 1000):
    for b in sorted(x for x in primes if x <= 1000):
        c = 0
        for i in range(1, 1000):
            p = i * i + a * i + b
            if p in primes:
                c += 1
            else:
                break
        if c > max_primes:
            max_primes = c
            coefficients = a * b


print(coefficients)