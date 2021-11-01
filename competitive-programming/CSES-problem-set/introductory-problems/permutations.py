n = int(input())

if n == 1:
    print(1, end='')
elif n <= 3:
    print('NO SOLUTION')
elif n == 4:
    print('3 1 4 2', end='')
else:
    for i in range(1, n + 1, 2):
        print(i, end=' ')
    for i in range(2, n + 1, 2):
        print(i, end=' ')
print()
