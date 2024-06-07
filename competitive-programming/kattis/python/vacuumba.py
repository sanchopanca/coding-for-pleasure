from math import radians, sin, cos

n = int(input())

for _ in range(n):
    angle = 90
    pos = [0, 0]
    for _ in range(int(input())):
        rotation, distance = map(float, input().split())
        angle += rotation
        pos[0] += distance * cos(radians(angle))
        pos[1] += distance * sin(radians(angle))
    print(*pos)
