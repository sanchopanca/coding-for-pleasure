from functools import cache


@cache
def path(x, y):
    if x == 0 or y == 0:
        return 1
    else:
        return path(x - 1, y) + path(x, y - 1)

print(path(20, 20))
