def sieve(up_to):
    is_prime = [True] * (up_to + 1)
    is_prime[0] = is_prime[1] = False
    for i, p in enumerate(is_prime):
        if p:
            for j in range(2 * i, up_to + 1, i):
                is_prime[j] = False
    t_primes = set()
    for i, p in enumerate(is_prime):
        if p:
            t_primes.add(i * i)
    return t_primes


def main():
    n = int(input())
    x = map(int, input().split())
    t_primes = sieve(int(1e6 + 7))
    for i in x:
        if i in t_primes:
            print('YES')
        else:
            print('NO')


main()
