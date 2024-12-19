from functools import cache

with open("../input/19.txt") as f:
    content = f.read()
patterns, designs = content.split("\n\n")
patterns = patterns.split(", ")
designs = designs.split()


@cache
def possible(design: str) -> bool:
    if not design:
        return True
    if design in patterns:
        return True
    for pattern in patterns:
        if pattern in design:
            left, right = design.split(pattern, maxsplit=1)
            if possible(left) and possible(right):
                return True
    return False


@cache
def how_many_possible(design: str) -> int:
    if not design:
        return 1
    result = 0
    for pattern in patterns:
        if design.startswith(pattern):
            _, right = design.split(pattern, maxsplit=1)
            result += how_many_possible(right)
    return result


print(sum(possible(design) for design in designs))
print(sum(how_many_possible(design) for design in designs))
