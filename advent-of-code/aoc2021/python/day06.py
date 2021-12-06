from functools import cache

with open('../input/06.txt') as f:
    numbers = list(map(int, f.readline().strip().split(',')))


@cache
def model1(time_to_reproduce, days):
    if time_to_reproduce == days - 1:
        return 2
    if time_to_reproduce > days - 1:
        return 1
    return model1(7, days - time_to_reproduce) + model1(9, days - time_to_reproduce)


def model_many(numbers, days):
    return sum(model1(n, days) for n in numbers)


def part1():
    print(model_many(numbers, 80))


def part2():
    print(model_many(numbers, 256))


part1()
part2()
