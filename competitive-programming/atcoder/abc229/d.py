def main():
    s = list(input().strip())
    s.append('.')
    k = int(input())
    cur_res = 0
    max_res = 0
    l, r = 0, 0
    curr_holes = 0
    while r < len(s):
        r += 1
        if s[r - 1] == 'X':
            max_res = max(max_res, r - l)
            # print(l, r, max_res)
        else:
            curr_holes += 1
            if curr_holes > k:
                while curr_holes > k:
                    if s[l] == '.':
                        curr_holes -= 1
                    l += 1
            max_res = max(max_res, r - l)
    print(min(max(max_res, k), len(s) - 1))


main()
