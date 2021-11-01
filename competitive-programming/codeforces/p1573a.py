t = int(input())
for i in range(t):
    result = 0
    n = int(input())
    number_str = input()
    last_digit = int(number_str[-1])
    result += last_digit
    for digit in number_str[:-1]:
        if digit == '0':
            continue
        result += 1 + int(digit)
    print(result)

# the digit in the last position takes the `digit` number of operations to get
# to zero. in ever other position non-zero digits need one additional
# operation to get to the last postion to go through the same treatment
