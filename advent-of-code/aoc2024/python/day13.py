from decimal import Decimal
import re


def is_int(num: Decimal):
    return abs(num - num.to_integral_value()) < 1e-8


def solve_equation(c11, c12, c1, c21, c22, c2):
    b = (c2 - (c21 * c1) / c11) / (c22 - c21 * c12 / c11)
    a = (c1 - c12 * b) / c11
    if is_int(a) and is_int(b):
        return int(a.to_integral_value()), int(b.to_integral_value())
    return None


pattern = r"[XY][+=](\d+)"

with open("../input/13.txt") as f:
    machines = f.read().split("\n\n")
result = 0
result2 = 0
for machine in machines:
    numbers = re.findall(pattern, machine)
    (c11, c21, c12, c22, c1, c2) = map(Decimal, numbers)
    answer = solve_equation(c11, c12, c1, c21, c22, c2)
    answer2 = solve_equation(
        c11, c12, c1 + Decimal(10000000000000), c21, c22, c2 + Decimal(10000000000000)
    )
    if answer:
        (a, b) = answer
        print(a, b)
        result += a * 3 + b
    if answer2:
        (a, b) = answer2
        print(a, b)
        result2 += a * 3 + b

print(result)
print(result2)
