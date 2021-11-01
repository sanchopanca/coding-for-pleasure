RNA = input().strip()

current_base = 'Z'
max_length = 0
current_length = 0
for base in RNA:
    if base != current_base:
        current_base = base
        current_length = 1
    else:
        current_length += 1
    max_length = max(max_length, current_length)
print(max_length)
