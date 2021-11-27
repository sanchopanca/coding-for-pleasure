field = []
n = int(input())


def move_piece_forward(i, j):
    p = field[i][j]
    field[i][j] = '.'
    field[i-1][j] = p


def remove_piece(i, j):
    field[i][j] = '.'


def print_field():
    for row in field:
        print(''.join(row))


def move(me, opponent):
    unblocks_me = None
    unblocks_opponent = None
    unblocks_nobody = None
    for i in range(1, n):
        for j in range(n):
            if field[i][j] == me and field[i-1][j] == '.':
                if i == n - 1 or field[i+1][j] == '.':
                    unblocks_nobody = (i, j)
                elif field[i+1][j] == opponent:
                    unblocks_opponent = (i, j)
                elif field[i+1][j] == me:
                    unblocks_me = (i, j)
    if unblocks_me:
        move_piece_forward(*unblocks_me)
        return True
    if unblocks_opponent:
        move_piece_forward(*unblocks_opponent)
        return True
    if unblocks_nobody:
        move_piece_forward(*unblocks_nobody)
        return True

    unblocks_me_can_move = None
    unblocks_me_cant_move = None
    unblocks_opponent_can_move = None
    unblocks_opponent_cant_move = None
    unblocks_nobody_can_move = None
    unblocks_nobody_cant_move = None
    for i in range(0, n):
        for j in range(n):
            if field[i][j] == opponent:
                if i == n - 1 or field[i + 1][j] == '.':
                    if i == 0 or field[i - 1][j] != '.':
                        unblocks_nobody_cant_move = (i, j)
                    else:
                        unblocks_nobody_can_move = (i, j)
                elif field[i + 1][j] == opponent:
                    if i == 0 or field[i - 1][j] != '.':
                        unblocks_opponent_cant_move = (i, j)
                    else:
                        unblocks_opponent_can_move = (i, j)
                elif field[i + 1][j] == me:
                    if i == 0 or field[i - 1][j] != '.':
                        unblocks_me_cant_move = (i, j)
                    else:
                        unblocks_me_can_move = (i, j)

    if unblocks_me_can_move:
        remove_piece(*unblocks_me_can_move)
        return True
    if unblocks_me_cant_move:
        remove_piece(*unblocks_me_cant_move)
        return True
    if unblocks_nobody_can_move:
        remove_piece(*unblocks_nobody_can_move)
        return True
    if unblocks_nobody_cant_move:
        remove_piece(*unblocks_nobody_cant_move)
        return True
    if unblocks_opponent_can_move:
        remove_piece(*unblocks_opponent_can_move)
        return True
    if unblocks_opponent_cant_move:
        remove_piece(*unblocks_opponent_cant_move)
        return True
    return False


def main():
    for _ in range(n):
        field.append(list(input().strip()))

    me = 'W'
    opponent = 'B'
    while move(me, opponent):
        # print('---------')
        # print(f'{me} moves:')
        # print_field()
        me, opponent = opponent, me

    if me == 'W':
        print('Snuke')
    else:
        print('Takahashi')



main()
