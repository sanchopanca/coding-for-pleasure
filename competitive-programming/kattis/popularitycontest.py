n, m = map(int, input().split())
friends = [0] * (n+1)
for _ in range(m):
    f1, f2 = map(int, input().split())
    friends[f1] += 1
    friends[f2] += 1

result = []
for success in range(1, n+1):
    result.append(friends[success] - success)
print(*result)
