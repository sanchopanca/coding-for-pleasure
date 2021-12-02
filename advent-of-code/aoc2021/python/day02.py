def part1():
    with open('../input/02.txt') as f:
        x, y = 0, 0
        for line in f:
            direction, distance = line.strip().split(' ')
            distance = int(distance)
            if direction == 'down':
                y += distance
            elif direction == 'up':
                y -= distance
            elif direction == 'forward':
                x += distance
        print(x * y)


def part2():
    with open('../input/02.txt') as f:
        x, y = 0, 0
        aim = 0
        for line in f:
            direction, distance = line.strip().split(' ')
            distance = int(distance)
            if direction == 'down':
                aim += distance
            elif direction == 'up':
                aim -= distance
            elif direction == 'forward':
                x += distance
                y += aim * distance
        print(x * y)


part1()
part2()
