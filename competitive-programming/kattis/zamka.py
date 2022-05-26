l = int(input())
d = int(input())
x = int(input())

numbers = []

for i in range(l, d+1):
    if sum(map(int, str(i))) == x:
        numbers.append(i)

print(numbers[0])
print(numbers[-1])
