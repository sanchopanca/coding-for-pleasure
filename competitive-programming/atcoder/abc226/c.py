n = int(input())

moves = {}
for i in range(n):
    t, _, *a = list(map(int, input().split()))
    moves[i+1] = {'dep': set(a), 't': t}

to_visit = {n}
visited = set()
total_time = 0
deps = set()
while len(to_visit) > 0:
    next_move = to_visit.pop()
    if next_move in visited:
        continue
    visited.add(next_move)
    total_time += moves[next_move]['t']
    to_visit.update(moves[next_move]['dep'])
print(total_time)




