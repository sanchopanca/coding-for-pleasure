def sieve(n):
    s = [True] * (n + 1)
    for i in range(2, n + 1):
        if not s[i]:
            continue
        for j in range(i + i, n + 1, i):
            s[j] = False
    return {i for i in range(2, n + 1) if s[i]}


primes = sieve(1_000_000)


def is_truncatable_prime(n):
    if n < 10:
        return False
    n *= 10
    while n := int('0' + str(n)[:-1]):
        if n not in primes:
            return False
    return True

c = 0
s = 0
for p in primes:
    if is_truncatable_prime(p):
        c += 1
        s += p
print(c)
assert(c == 11)
print(s)