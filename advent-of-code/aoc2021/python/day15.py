def dijkstra(cave):
    start = (0, 0)
    N, M = len(cave), len(cave[0])
    end = (N - 1, M - 1)
    visited = set()
    distances = [[float('inf') for _ in range(M)] for _ in range(N)]
    distances[start[0]][start[1]] = 0
    unvisited = {start}
    while unvisited:
        current = min(unvisited, key=lambda x: distances[x[0]][x[1]])
        unvisited.remove(current)
        x, y = current
        neighbors = set()
        for i, j in {(-1, 0), (1, 0), (0, -1), (0, 1)}:
            if 0 <= x + i < N and 0 <= y + j < M:
                neighbors.add((x + i, y + j))
                distances[x + i][y + j] = min(distances[x + i][y + j], distances[x][y] + cave[x + i][y + j])
        visited.add(current)
        for n in neighbors:
            if n not in visited:
                unvisited.add(n)
    print(distances[end[0]][end[1]])


def get_cave():
    with open('../input/15.txt') as f:
        return [list(map(int, line.strip())) for line in f]


def expand_part(cave, big_cave, off_x, off_y, a):
    for i in range(len(cave)):
        for j in range(len(cave[0])):
            big_cave[i + off_x][j + off_y] = (cave[i][j] + a - 1) % 9 + 1


def expand_cave(cave):
    N, M = len(cave), len(cave[0])
    big_cave = [[0 for _ in range(M * 5)] for _ in range(N * 5)]
    for i in range(N):
        for j in range(M):
            big_cave[i][j] = cave[i][j]
    for i in range(5):
        for j in range(5):
            expand_part(cave, big_cave, i * N, j * M, i + j)
    return big_cave


def part1():
    dijkstra(get_cave())


def part2():
    dijkstra(expand_cave(get_cave()))


part1()
part2()
