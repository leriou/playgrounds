import random


class Treap:

    def __init__(self):
        self.left = None
        self.right = None
        self.value = None
        self.fix = random.Random().randint(-10000, -1)

    def left_rotate(self):
        right = self.right
        self.right = right.left
        right.left = self
        self = right
        return self

    def right_rotate(self):
        left = self.left
        self.left = left.right
        left.right = self
        self = left
        return self

    def insert(self, value):
        if self.value is None:
            self.value = value
            self.left = Treap()
            self.right = Treap()
            self.fix = random.Random().randint(0, 10000)
        elif value <= self.value:
            self.left.insert(value)
            if self.left.fix < self.fix:
                self.right_rotate()
        else:
            self.right.insert(value)
            if self.right.fix < self.fix:
                self.left_rotate()
        return self

    def delete(self, value):
        if value == self.value:
            if self.right is None:
                self = self.right
            elif self.right is None:
                self = self.left
            else:
                if self.left.fix < self.right.fix:
                    self.right_rotate()
                    self.right.delete(value)
                else:
                    self.left_rotate()
                    self.left.delete(value)
        elif value < self.value:
            self.left.delete(value)
        else:
            self.right.delete(value)
        return self

    def collect(self, level=0, arr={}):
        if self.value is not None:
            if arr.get(level) is None:
                arr[level] = [(self.value, self.fix)]
            else:
                arr[level].append((self.value, self.fix))
            self.arr = arr
            level += 1
            self.left.collect(level)
            self.right.collect(level)
        return self

    def pretty(self):
        print(self.arr)


r = Treap()
r.insert(1)
r.insert(6)
r.insert(8)
r.insert(3)
r.insert(4)
r.insert(11)
r.insert(28)

r.collect().pretty()
