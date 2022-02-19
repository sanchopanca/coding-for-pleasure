def inp():
    with open('../input/17.txt') as f:
        _, _, x, y = f.read().strip().split()
        x = x[2:-1]
        y = y[2:]
        x = [int(i) for i in x.split('..')]
        y = [int(i) for i in y.split('..')]
    return x, y


def y_hits(y_min, y_max, y_speed):
    y_range = range(y_min, y_max + 1)
    y = 0
    while y >= y_min:
        y += y_speed
        y_speed -= 1
        if y in y_range:
            return True
    return False


def max_y(y_speed):
    y = 0
    while y_speed > 0:
        y += y_speed
        y_speed -= 1
    return y


def hits(x_min, x_max, x_speed, y_min, y_max, y_speed):
    x_range = range(x_min, x_max + 1)
    y_range = range(y_min, y_max + 1)
    x = 0
    y = 0
    while x <= x_max and y >= y_min:
        x += x_speed
        if x in x_range and y in y_range:
            print('speed:', x_speed, y_speed)
            return True
        y += y_speed
        if x in x_range and y in y_range:
            print('speed:', x_speed, y_speed)
            return True
        x_speed -= 1
        x_speed = max(x_speed, 0)
        y_speed -= 1
    return False


def part1():
    _, y = inp()
    vel = 1000
    while not y_hits(y[0], y[1], vel):
        vel -= 1
    print(max_y(vel))


def part2():
    x, y = inp()
    s = 0
    for x_speed in range(400):
        for y_speed in range(-100, 0):
            if hits(x[0], x[1], x_speed, y[0], y[1], y_speed):
                print(x_speed, y_speed)
                s += 1
    print(s)


part1()
part2()
