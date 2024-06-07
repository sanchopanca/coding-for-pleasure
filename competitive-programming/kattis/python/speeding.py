n = int(input())

shots = []
for _ in range(n):
    t, d = map(int, input().split())
    shots.append((t, d))

speed = 0

for x in zip(shots[:-1], shots[1:]):
    (t1, d1), (t2, d2) = x
    speed = max(speed, (d2 - d1) // (t2 - t1))

print(speed)
