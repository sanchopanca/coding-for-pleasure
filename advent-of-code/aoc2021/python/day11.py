from collections import defaultdict


def step(cavern):
    n = 0
    flashed = set()
    for i in range(10):
        for j in range(10):
            cavern[(i, j)] += 1
    flashed_this_time = True
    while flashed_this_time:
        flashed_this_time = False
        for i in range(10):
            for j in range(10):
                if cavern[(i, j)] > 9 and not (i, j) in flashed:
                    flashed_this_time = True
                    flashed.add((i, j))
                    for x in range(i-1, i+2):
                        for y in range(j-1, j+2):
                            cavern[(x, y)] += 1
    for i in range(10):
        for j in range(10):
            if cavern[(i, j)] > 9:
                cavern[(i, j)] = 0
                n += 1
    return n


def part1():
    with open('../input/11.txt') as f:
        inp = [list(map(int, line.strip())) for line in f]
    cavern = defaultdict(lambda: -1e9)
    for y, row in enumerate(inp):
        for x, val in enumerate(row):
            cavern[(x, y)] = val

    flashes = 0
    for i in range(1000):
        flashes_this_step = step(cavern)
        flashes += flashes_this_step
        if i == 99:
            print(flashes)
        if flashes_this_step == 100:
            print(i+1)
            break


part1()
