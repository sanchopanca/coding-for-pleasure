def is_abundant(n):
    i = 2
    s = {1}
    while i * i <= n:
        if n % i == 0:
            s.add(i)
            s.add(n//i)
        i += 1
    return sum(s) > n


N = 28123

abundant_numbers = {x for x in range(12, N) if is_abundant(x)}

result = 0
for i in range(1, N):
    for j in range(1, i):
        if j in abundant_numbers and i - j in abundant_numbers:
            break
    else:
        result += i

print(result)
