from collections import defaultdict


def parse_graph():
    graph = defaultdict(list)
    with open('../input/12.txt') as f:
        for line in f:
            line = line.strip()
            a, b = line.split('-')
            graph[a].append(b)
            graph[b].append(a)
    return graph


def part1():
    graph = parse_graph()
    paths = set()
    stack = [['start', ['start']]]
    while stack:
        node, path = stack.pop()
        if node == 'end':
            paths.add(tuple(path))
            continue
        for n in graph[node]:
            if n.islower() and n in path:
                continue
            stack.append([n, path + [n]])
    print(len(paths))


def part2():
    graph = parse_graph()
    paths = set()
    stack = [['start', ['start'], False]]
    while stack:
        node, path, has_double = stack.pop()
        if node == 'end':
            paths.add(tuple(path))
            continue
        for n in graph[node]:
            repeated_lower = n in path and n.islower()
            if repeated_lower and has_double and n.islower() or n == 'start':
                continue
            stack.append([n, path + [n], repeated_lower or has_double])
    print(len(paths))


part1()
part2()
