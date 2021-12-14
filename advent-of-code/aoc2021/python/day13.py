def make_paper(dots, max_x, max_y):
    dots = set(dots)
    paper = []
    for x in range(max_x+1):
        paper.append([False] * (max_y+1))
    for x, y in dots:
        paper[x][y] = True
    return paper


def visualize(paper):
    s = ''
    for j in range(len(paper[0])):
        for i in range(len(paper)):
            if paper[i][j]:
                s += '█'
            else:
                s += ' '
        s += '\n'
    print(s)


def fold_along_x(paper, x_coord):
    for y in range(len(paper[0])):
        for x in range(x_coord):
            paper[x][y] = paper[x][y] or paper[-1 - x][y]
    for x in range(len(paper) - 1, x_coord - 1, -1):
        del paper[x]
    return paper


def fold_along_y(paper, y_coord):
    for x in range(len(paper)):
        for y in range(y_coord):
            paper[x][y] = paper[x][y] or paper[x][-1 - y]

        for y in range(len(paper[x]) - 1, y_coord - 1, -1):
            del paper[x][y]
    return paper


def count(paper):
    return sum(sum(x) for x in paper)


def part1and2():
    with open('../input/13.txt') as f:
        data = f.read()
    data.strip()
    paper_dots, instructions = data.split('\n\n')
    paper_dots = [tuple(map(int, line.split(','))) for line in paper_dots.split('\n')]
    instructions = [x.split(' ')[-1] for x in instructions.split('\n')]
    instructions.pop()
    instructions = [(x.split('=')[0], int(x.split('=')[1])) for x in instructions]
    max_x, max_y = 0, 0
    for x, y in paper_dots:
        max_x = max(max_x, x)
        max_y = max(max_y, y)
    paper = make_paper(paper_dots, max_x, max_y)
    instruction = instructions.pop(0)
    fold_along_x(paper, instruction[1])
    print(count(paper))
    for axis, coord in instructions:
        if axis == 'x':
            paper = fold_along_x(paper, coord)
        else:
            paper = fold_along_y(paper, coord)

    visualize(paper)


part1and2()
