from typing import Union


class Pair:
    def __init__(self, left: 'Element', right: 'Element', parent: 'Pair' = None, level: int = 0):
        self.left = left
        self.right = right
        self.parent = parent
        self.level = level

    def explosive(self):
        if self.level == 4:
            return True
        if isinstance(self.left, Pair):
            if self.left.explosive():
                return True
        if isinstance(self.right, Pair):
            if self.right.explosive():
                return True
        return False

    def explode(self):
        if self.level != 3:

    def traverse(self):
        def tr(el: 'Element'):
            if isinstance(el, Pair):
                yield from tr(el.left)
                yield from tr(el.right)
            else:
                yield el
        yield from tr(self)

    @staticmethod
    def from_list(lst: list, parent: 'Pair' = None, level: int = 0) -> 'Pair':
        el1, el2 = lst
        p = Pair(0, 0, parent=parent, level=level)
        if isinstance(el1, list):
            el1 = Pair.from_list(el1, p, level + 1)
        if isinstance(el2, list):
            el2 = Pair.from_list(el2, p, level + 1)
        p.left = el1
        p.right = el2
        return p

    def to_list(self) -> list:
        l, r = self.left, self.right
        if isinstance(l, Pair):
            l = l.to_list()
        if isinstance(r, Pair):
            r = r.to_list()
        return [l, r]

    def __str__(self):
        return str(self.to_list())


Element = Union[int, Pair]

p = Pair.from_list([1, [[2, 3], 5]])
p1 = Pair.from_list([[[[[9,8],1],2],3],4])
p2 = Pair.from_list([[[[0,9],2],3],4])

print(p.explosive())
print(p1.explosive())
print(p2.explosive())
# print([x for x in p.traverse()])
# print(Pair.from_list([1, [[2, 3], 5]]))
