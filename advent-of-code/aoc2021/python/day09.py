from collections import defaultdict


def part1and2():
    heightmap = []
    with open("../input/09.txt") as f:
        for line in f:
            heightmap.append(list(map(int, line.strip())))
    n = len(heightmap)
    m = len(heightmap[0])
    risk = 0
    heightmap_for_lazy_people = defaultdict(lambda: 10)
    for i in range(n):
        for j in range(m):
            heightmap_for_lazy_people[(i, j)] = heightmap[i][j]

    low_points = []
    for i in range(n):
        for j in range(m):
            if (
                heightmap[i][j] < heightmap_for_lazy_people[(i, j - 1)]
                and heightmap[i][j] < heightmap_for_lazy_people[(i, j + 1)]
                and heightmap[i][j] < heightmap_for_lazy_people[(i - 1, j)]
                and heightmap[i][j] < heightmap_for_lazy_people[(i + 1, j)]
            ):
                risk += heightmap[i][j] + 1
                low_points.append((i, j))
    print(risk)

    sizes = []
    for x, y in low_points:
        size = 0
        queue = [(x, y)]
        visited = set()
        while queue:
            i, j = queue.pop(0)
            if (i, j) in visited:
                continue
            if heightmap_for_lazy_people[(i, j)] > 8:
                continue
            visited.add((i, j))
            size += 1
            queue.append((i, j - 1))
            queue.append((i, j + 1))
            queue.append((i - 1, j))
            queue.append((i + 1, j))
        sizes.append(size)

    sizes.sort(reverse=True)
    print(sizes[0] * sizes[1] * sizes[2])


part1and2()
