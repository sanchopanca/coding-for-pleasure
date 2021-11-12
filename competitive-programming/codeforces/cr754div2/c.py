def dominant_character(s, n):
    # results = set()
    result = n + 1
    if 'abbacca' in s or 'accabba' in s:
        result = 7
    i = s.find('a')
    if i == -1:
        return -1
    start = i
    i += 1
    n_a = 1
    n_b = 0
    n_c = 0
    while i < len(s):
        if start != -1:
            if s[i] == 'a':
                n_a += 1
            elif s[i] == 'b':
                n_b += 1
            else:
                n_c += 1
            if n_a > n_b and n_a > n_c:
                result = min(i-start+1, result)
                start = i
                n_b = 0
                n_c = 0
                n_a = 1
            elif n_b > n_a or n_c > n_a:
                start = -1
                n_a, n_b, n_c = 0, 0, 0
        else:
            if s[i] == 'a':
                n_a = 1
                start = i
        i += 1
    if result == n + 1:
        return -1
    return result




def main():
    t = int(input())
    # t = 1
    for _ in range(t):
        n = int(input())
        s = input().strip()
        # s = 'a' + 'c' * 1000000
        # n = len(s)
        print(dominant_character(s, n))


main()
