x, y = 1, 2

s = 0
while y <= 4_000_000:
    x, y = y, x + y
    if x % 2 == 0:
        s += x
print(s)
