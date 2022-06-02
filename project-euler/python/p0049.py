from itertools import permutations, combinations


def sieve(n):
    s = [True] * (n + 1)
    for i in range(2, n + 1):
        if not s[i]:
            continue
        for j in range(i + i, n + 1, i):
            s[j] = False
    return {i for i in range(2, n + 1) if s[i]}


def p2n(p):
    return int(''.join(p))


primes = sieve(10_000)
candidates = set()
for n in range(1000, 10_000):
    perms = permutations(sorted(str(n)))
    c = 0
    candidate = []
    for p in perms:
        if p2n(p) in primes:
            candidate.append(p2n(p))
    if len(set(candidate)) >= 3:
        candidates.add(tuple(sorted(set(candidate))))

for c in candidates:
    for triple in combinations(c, 3):
        if c[0] == 1487 or c[0] < 1000:
            continue
        if triple[1] - triple[0] == triple[2] - triple[1]:
            print(*triple, sep='')

