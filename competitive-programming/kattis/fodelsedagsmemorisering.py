n = int(input())

d = {}
for i in range(n):
    name, score, date = input().strip().split()
    score = int(score)
    if date not in d:
        d[date] = (name, score)
    else:
        if score > d[date][1]:
            d[date] = (name, score)
names = [x[0] for x in d.values()]
print(len(names))
print(*sorted(names), sep='\n')
