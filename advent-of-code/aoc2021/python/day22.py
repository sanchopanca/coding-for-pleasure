def part1():
    with open('../input/22.txt') as f:
        lines = [line.strip() for line in f.readlines()]
    steps = []
    for line in lines:
        on_off, coordinates = line.split()
        x, y, z = coordinates.split(',')
        x = x[2:].split('..')
        y = y[2:].split('..')
        z = z[2:].split('..')
        steps.append((on_off == 'on', x, y, z))

    reactor = set()
    for i, step in enumerate(steps):
        print(i, '/', len(steps))
        on, x, y, z = step
        for i in range(int(x[0]), int(x[1]) + 1):
            if i not in range(-50, 51):
                continue
            for j in range(int(y[0]), int(y[1]) + 1):
                if j not in range(-50, 51):
                    continue
                for k in range(int(z[0]), int(z[1]) + 1):
                    if k not in range(-50, 51):
                        continue
                    if on:
                        reactor.add((i, j, k))
                    else:
                        reactor.discard((i, j, k))
    print(len(reactor))


part1()
