from copy import deepcopy

with open('../input/p067_triangle.txt') as f:
    triangle = [list(map(int, x.split())) for x in f]

paths = deepcopy(triangle)

for i in range(1, len(triangle)):
    for j in range(len(triangle[i])):
        if j - 1 < 0:
            paths[i][j] = paths[i-1][j] + triangle[i][j]
        elif j == len(triangle[i-1]):
            paths[i][j] = paths[i-1][j-1] + triangle[i][j]
        else:
            paths[i][j] = max(paths[i-1][j], paths[i-1][j-1]) + triangle[i][j]

print(max(paths[-1]))
