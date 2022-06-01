def number_of_divisors(n):
    divisors = set()
    i = 1
    while i * i <= n:
        if n % i == 0:
            divisors.add(i)
            divisors.add(n // i)
        i += 1
    return len(divisors)


n = 1
while True:
    t = n * (n + 1) // 2
    n += 1
    if number_of_divisors(t) > 500:
        print(t)
        break
