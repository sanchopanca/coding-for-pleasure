def part1():
    with open('../input/21.txt') as f:
        positions = [int(line[-2]) for line in f.readlines()]
    scores = [0, 0]
    rolls = 0
    die = 0
    player = 0
    while max(scores) < 1000:
        to_move = 0
        for _ in range(3):
            rolls += 1
            die = die % 100 + 1
            to_move += die
        positions[player] = (positions[player] + to_move - 1) % 10 + 1
        scores[player] += positions[player]
        player = (player + 1) % 2
    print(min(scores) * rolls)


part1()
