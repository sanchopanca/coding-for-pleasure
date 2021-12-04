def mark_board(board, marked_board, number):
    for i in range(len(board)):
        for j in range(len(board[0])):
            if board[i][j] == number:
                marked_board[i][j] = 1


def check_marked_board(marked_board):
    for row in marked_board:
        if sum(row) == len(row):
            return True
    for i in range(len(marked_board[0])):
        column = [row[i] for row in marked_board]
        if sum(column) == len(column):
            return True
    return False


def calculate_score(board, marked_board):
    score = 0
    for i in range(len(board)):
        for j in range(len(board[0])):
            if marked_board[i][j] == 0:
                score += board[i][j]
    return score


def board_score(board, numbers):
    steps = 0
    marked_board = [[0 for _ in range(len(board[0]))] for _ in range(len(board))]
    for number in numbers:
        steps += 1
        mark_board(board, marked_board, number)
        if check_marked_board(marked_board):
            return steps, number * calculate_score(board, marked_board)
    return steps, 0


def part1and2():
    with open('../input/04.txt') as f:
        lines = f.readlines()
    numbers = list(map(int, lines.pop(0).split(',')))
    boards = []
    while lines:
        lines.pop(0)
        board = []
        for i in range(5):
            board.append(list(map(int, lines.pop(0).split())))
        boards.append(board)
    min_steps, min_steps_score = 1e5, 0
    max_steps, max_steps_score = 0, 0
    for board in boards:
        steps, score = board_score(board, numbers)
        if steps < min_steps:
            min_steps = steps
            min_steps_score = score
        if steps > max_steps:
            max_steps = steps
            max_steps_score = score
    print(min_steps_score, max_steps_score)


part1and2()
