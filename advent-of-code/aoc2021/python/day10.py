def part1and2():
    with open('../input/10.txt') as f:
        program = [l.strip() for l in f.readlines()]

    opening_of = {')': '(', ']': '[', '}': '{', '>': '<'}
    opening = {'(', '[', '{', '<'}
    scores = {')': 3, ']': 57, '}': 1197, '>': 25137}
    scores2table = {'(': 1, '[': 2, '{': 3, '<': 4}
    score = 0
    scores2 = []
    for line in program:
        stack = []
        for c in line:
            if c in opening:
                stack.append(c)
            else:
                if stack[-1] == opening_of[c]:
                    stack.pop()
                else:
                    score += scores[c]
                    break
        else:
            score2 = 0
            for c in reversed(stack):
                score2 *= 5
                score2 += scores2table[c]
            scores2.append(score2)
    scores2.sort()
    print(score)
    print(scores2[len(scores2) // 2])


part1and2()
