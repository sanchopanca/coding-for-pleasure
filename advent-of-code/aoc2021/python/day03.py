def part1():
    with open('../input/03.txt') as f:
        z = zip(*list(map(str.strip, f.readlines())))
        gamma = [int(position.count('1') > position.count('0')) for position in z]

        epsilon = [1 if digit == 0 else 0 for digit in gamma]
        gamma = int(''.join(map(str, gamma)), 2)
        epsilon = int(''.join(map(str, epsilon)), 2)
        print(gamma * epsilon)


def p2(what_to_remove):
    with open('../input/03.txt') as f:
        values = list(map(str.strip, f.readlines()))
        for i in range(len(values[0])):
            position_values = [value[i] for value in values]
            to_remove = what_to_remove(position_values)
            for j in range(len(values) - 1, -1, -1):
                if int(values[j][i]) == to_remove:
                    del values[j]
            if len(values) == 1:
                return int(values[0], 2)


def part2():
    o2 = p2(lambda x: int(x.count('0') > x.count('1')))
    co2 = p2(lambda x: int(x.count('1') >= x.count('0')))
    print(o2 * co2)


part1()
part2()
