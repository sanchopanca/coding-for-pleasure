from collections import Counter


def print_result(chars):
    counts = sorted(chars.values())
    print(counts[-1] - counts[0])


def part1and2():
    with open('../input/14.txt') as f:
        lines = [line.strip() for line in f.readlines()]
    chain = lines.pop(0)
    lines.pop(0)
    rules = {}
    for line in lines:
        ab, c = line.split(' -> ')
        rules[ab] = c

    chars = Counter(chain)
    pairs = Counter()
    for a, b in zip(chain, chain[1:]):
        pairs[a+b] += 1
    for i in range(40):
        change = Counter()
        for ab, count in pairs.items():
            a, b = ab
            if rules[ab]:
                chars[rules[ab]] += count
                change[ab] -= count
                change[a+rules[ab]] += count
                change[rules[ab]+b] += count
        pairs.update(change)
        if i == 9:
            print_result(chars)
    print_result(chars)


part1and2()
