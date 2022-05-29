n = int(input())

d = {
    1: 1,
    2: 2,
    3: 4,
    4: 7,
}

for i in range(5, 25):
    d[i] = d[i-1] + d[i-2] + d[i-3]

print(d[n])
