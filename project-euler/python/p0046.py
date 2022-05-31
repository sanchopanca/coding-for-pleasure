def sieve(n):
    s = [True] * (n + 1)
    for i in range(2, n + 1):
        if not s[i]:
            continue
        for j in range(i + i, n + 1, i):
            s[j] = False
    return {i for i in range(2, n + 1) if s[i]}


m = 10000

double_squares = [2 * x * x for x in range(1, m)]
primes = sieve(m)

for n in range(3, m, 2):
    if n in primes:
        continue
    for d in double_squares:
        if n - d in primes:
            break
    else:
        print(n)
        break
