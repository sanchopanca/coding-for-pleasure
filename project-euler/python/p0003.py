n = 600851475143
factors = []

while True:
    for i in range(2, n):
        if n % i == 0:
            factors.append(i)
            n = n // i
            break
    else:
        factors.append(n)
        break

print(sorted(factors)[-1])
