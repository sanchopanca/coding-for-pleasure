s = 0

problems = input().strip().split(';')
for p in problems:
    if '-' in p:
        a, b = map(int, p.split('-'))
        s += b - a + 1
    else:
        s += 1
print(s)
