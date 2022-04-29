# 40 seconds
def square_digits(n):
    s = 0
    for d in str(n):
        d = int(d)
        s += d * d
    return s

result = 0
for i in range(1, 10_000_000):
    while i not in (1, 89):
        i = square_digits(i)
    if i == 89:
        result += 1

print(result)
