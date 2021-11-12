def reverse_binary_string(s, n):
    indexes_1 = []
    indexes_0 = []
    for i in range(n-1):
        if s[i] == '1':
            indexes_1.append(i)
    for i in range(n-1, 0, -1):
        if s[i] == '0':
            indexes_0.append(i)
    if len(indexes_1) == 0 or len(indexes_0) == 0:
        return []
    m = min(len(indexes_1), len(indexes_0))
    while m > 0 and indexes_1[m-1] > indexes_0[m-1]:
        m -= 1
    indx = indexes_1[:m] + list(reversed(indexes_0[:m]))
    return indx

def swap_bits(s, indexes):
    print(s)
    for i in range(len(indexes) // 2):
        s[indexes[i]], s[indexes[-i-1]] = s[indexes[-i-1]], s[indexes[i]]
        # s[i], s[len(s) - i - 1] = s[len(s) - i - 1], s[i]
    print(s)


def main():
    t = int(input())
    # t = 1
    for _ in range(t):
        n = int(input())
        s = input().strip()
        # s = '101010010101110001010100101111111000'
        # n = len(s)
        idx = reverse_binary_string(s, n)
        if len(idx) == 0:
            print(0)
            continue
        print(1)
        print(len(idx), end=' ')
        print(*[i+1 for i in idx])
        # swap_bits(list(s), idx)


main()
