n = int(input())
array = list(map(int, input().split()))

answer = 0
for i in range(1, n):
    new_value = max(array[i], array[i-1])
    answer += new_value - array[i]
    array[i] = new_value
print(answer)
