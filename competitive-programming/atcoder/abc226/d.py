from math import gcd


def reduce_pair(a, b):
    if a == 0:
        return 0, b // abs(b)
    if b == 0:
        return a // abs(a), 0
    divisor = gcd(a, b)
    return a // divisor, b // divisor


n = int(input())

cities = []
for i in range(n):
    x, y = list(map(int, input().split()))
    cities.append((x, y))
spells = set()
for i in range(len(cities)-1):
    for j in range(i+1, len(cities)):
        x1 = cities[i][0] - cities[j][0]
        y1 = cities[i][1] - cities[j][1]
        x1, y1 = reduce_pair(x1, y1)
        x2, y2 = -x1, -y1
        spells.add((x1, y1))
        spells.add((x2, y2))

print(len(spells))
# print(spells)
