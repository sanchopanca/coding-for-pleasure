t = int(input())


def fill_lists(numbers, colors):
    go_up, go_down = [], []
    for i in range(len(numbers)):
        if colors[i] == 'R':
            if numbers[i] > len(numbers):
                return [], [], False
            go_up.append(numbers[i])
        else:
            if numbers[i] < 1:
                return [], [], False
            go_down.append(numbers[i])
    return go_up, go_down, True


for _ in range(t):
    n = int(input())
    numbers = list(map(int, input().split()))
    colors = input().strip()
    go_up, go_down, success = fill_lists(numbers, colors)
    if not success:
        print('NO')
        continue
    go_up.sort(reverse=True)
    go_down.sort()
    filling = n
    done = False
    for item in go_up:
        if item <= filling:
            filling -= 1
        else:
            print('NO')
            done = True
            break
    if done:
        continue
    filling = 1
    for item in go_down:
        if item >= filling:
            filling += 1
        else:
            print('NO')
            done = True
            break
    if not done:
        print('YES')


