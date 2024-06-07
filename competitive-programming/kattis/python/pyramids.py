n = int(input())

layer = 1
side = 1
while n > 0:
    blocks = side * side
    if n >= blocks:
        layer += 1
        side += 2
    n -= blocks
print(layer - 1)
