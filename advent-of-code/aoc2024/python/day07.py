from itertools import chain, zip_longest, product


class ElfInt(int):
    def __mul__(self, other):
        return ElfInt(int(self) * int(other))

    def __truediv__(self, other):
        return ElfInt(self + other)

    def __mod__(self, other):
        return ElfInt(int(str(self) + str(other)))


# intertwine(['1', '2', '3'], ['+', '*']) returns '1+2*3'
def intertwine(seq1, seq2):
    return "".join(chain.from_iterable(zip_longest(seq1, seq2, fillvalue="")))


def solve(ops):
    with open("../input/07.txt") as f:
        lines = f.readlines()
    answer = 0
    total = len(lines)
    for i, line in enumerate(lines):
        progress = 100 * i / total
        print(f"{progress:.2f}%")
        line.strip()
        result, params = line.split(": ")
        result = int(result)
        params = list(map(lambda x: f"ElfInt({x})", params.split()))
        operators_combination = product(ops, repeat=len(params) - 1)
        for operators in operators_combination:
            if result == eval(intertwine(params, operators)):
                answer += result
                break
    print(answer)


if __name__ == "__main__":
    solve(["*", "/"])
    solve(["*", "/", "%"])
