n = int(input())

numbers = []
for i in range(n):
    numbers.append(int(input()))

print(*reversed(numbers), sep='\n')
