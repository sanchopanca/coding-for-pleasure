from collections import Counter


def part1and2():
    with open('../input/05.txt') as f:
        lines = []
        diag_lines = []
        for line in f:
            x1, y1, x2, y2 = map(int, line.strip().replace('-', '').replace('>', '').replace(',', ' ').split())
            if x1 != x2 and y1 != y2:
                diag_lines.append((x1, y1, x2, y2))
            else:
                lines.append((x1, y1, x2, y2))
    floor = Counter()
    for x1, y1, x2, y2 in lines:
        if x1 == x2:
            for y in range(min(y1, y2), max(y1, y2) + 1):
                floor[(x1, y)] += 1
        else:
            for x in range(min(x1, x2), max(x1, x2) + 1):
                floor[(x, y1)] += 1
    result = 0
    for v in floor.values():
        if v > 1:
            result += 1
    print(result)

    for x1, y1, x2, y2 in diag_lines:
        if (x1, y1) > (x2, y2):
            x1, y1, x2, y2 = x2, y2, x1, y1
        for i in range(x2 - x1 + 1):
            if y1 < y2:
                floor[(x1 + i, y1 + i)] += 1
            else:
                floor[(x1 + i, y1 - i)] += 1

    result = 0
    for v in floor.values():
        if v > 1:
            result += 1
    print(result)


part1and2()
