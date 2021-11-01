primes: list = []


def main():
    n = int(input())
    global primes
    primes = find_primes(n)
    factors = factorize(n)
    for p in primes:
        if p not in factors:
            print(p)
            return


def find_primes(max_n):
    numbers = {nmbr: True for nmbr in range(2, max_n+1)}
    for nmbr, is_prime in numbers.items():
        if is_prime:
            step = nmbr
            nmbr += step
            while nmbr <= max_n:
                numbers[nmbr] = False
                nmbr += step
    return [nmbr for nmbr, is_prime in numbers.items() if is_prime]


def factorize(nmbr):
    factors = set()
    for p in primes:
        if nmbr % p == 0:
            factors.add(p)
            nmbr = nmbr // p
        if nmbr == 1:
            break
    return factors


main()
