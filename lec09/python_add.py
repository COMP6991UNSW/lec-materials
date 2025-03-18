class Point:
    x: int
    y: int

    def __add__(self, rhs):
        Point(self.x + rhs.x, self.y + rhs.y)
