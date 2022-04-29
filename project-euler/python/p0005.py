# time: 1 min
n = 2520
while True:
    for d in range(2, 21):
        if n % d != 0:
            n += 1
            break
    else:
        print(n)
        break
