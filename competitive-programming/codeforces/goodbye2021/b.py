import random
import string


def true_solution(s):
    potentials = []
    for i in range(0, len(s)):
        potentials.append(s[:i+1] + s[:i+1][::-1])
    # print(potentials)
    return min(potentials)


def main():
    t = int(input())
    for _ in range(t):
        n = int(input())
        s = input().strip()
        # n = 5
        # s = ''.join(random.choice(string.ascii_lowercase) for x in range(n))
        k = -1
        prev = None
        for i in range(n-1):
            if s[i] < s[i+1]:
                k = i
                break
            if s[i] == s[i+1]:
                for j in range(1, n):
                    # if i + 1 + j >= n:
                    #     break
                    # if i - j < 0:
                    #     k = i
                    #     break
                    # if s[i-j] < s[i + 1]:
                    #     k = i
                    #     break
                    if prev is None or prev < s[i+1]:
                        k = i
                        break
                    else:
                        break
                    # if s[i-j] > s[i + 1]:
                    #     break
                if k != -1:
                    break
            else:
                prev = s[i]
        else:
            k = n - 1
            # print('here')
        # print(_, k)
        sol = s[:k + 1] + s[:k + 1][::-1]
        print(sol)
        # if sol != true_solution(s):
        #     print('Wrong answer')
        #     print('TRUE: ', true_solution(s))
        #     print('Orginal string', s)



main()
