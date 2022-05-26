numbers = list(map(int, input().strip()))

for bit in range(3, -1, -1):
    for i, n in enumerate(numbers):
        mask = 1 << bit
        if n & mask:
            print('*', end='')
        else:
            print('.', end='')
        if i < 3:
            print(' ', end='')
        if i == 1:
            print('  ', end='')
    print()
