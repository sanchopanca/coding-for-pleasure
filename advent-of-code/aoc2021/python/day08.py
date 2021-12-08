from collections import defaultdict


def part1():
    result = 0
    with open('../input/08.txt') as f:
        for line in f:
            digits = line.strip().split('|')[-1].split()
            for digit in digits:
                if len(digit) in {2, 3, 4, 7}:
                    result += 1
    print(result)


def deduct(digits, number):
    d2s = {}  # digit -> segment letters
    l2d = defaultdict(set)  # length -> digits
    for digit in digits:
        l2d[len(digit)].add(digit)
        if len(digit) == 2:
            d2s[1] = set(digit)
        elif len(digit) == 3:
            d2s[7] = set(digit)
        elif len(digit) == 4:
            d2s[4] = set(digit)
        elif len(digit) == 7:
            d2s[8] = set(digit)

    for digit in l2d[6]:  # 0, 6, 9
        if (d2s[8] - set(digit)).pop() not in d2s[4]:
            d2s[9] = set(digit)
        elif (d2s[8] - set(digit)).pop() not in d2s[1]:
            d2s[0] = set(digit)
        else:
            d2s[6] = set(digit)

    for digit in l2d[5]:  # 2, 3, 5
        if len(d2s[7] & set(digit)) == 3:
            d2s[3] = set(digit)
        elif len(d2s[4] & set(digit)) == 2:
            d2s[2] = set(digit)
        else:
            d2s[5] = set(digit)

    s2d = {}
    for k, v in d2s.items():
        s2d[frozenset(v)] = k
    n = ''
    for digit in number:
        n += str(s2d[frozenset(digit)])
    return int(n)


def part2():
    s = 0
    with open('../input/08.txt') as f:
        for line in f:
            all_digits, number = map(str.split, line.strip().split('|'))
            s += deduct(all_digits, number)
    print(s)


part1()
part2()
