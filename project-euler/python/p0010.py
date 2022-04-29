def sieve(n):
    s = [True] * (n + 1)
    for i in range(2, n + 1):
        if not s[i]:
            continue
        for j in range(i + i, n + 1, i):
            s[j] = False
    return [i for i in range(2, n + 1) if s[i]]

print(sum(sieve(2_000_000)))
