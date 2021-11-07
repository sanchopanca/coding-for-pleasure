n = int(input())
s = set()
for i in range(n):
    s.add(tuple(map(int, input().split())))

print(len(s))
