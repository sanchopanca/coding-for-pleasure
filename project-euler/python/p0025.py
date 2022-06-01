a, b = 1, 1
i = 2
while len(str(a)) < 1000:
    a, b = a + b, a
    i += 1
print(i)