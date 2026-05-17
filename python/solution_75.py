import collections
import itertools
import math


def solution() -> None:
    ceiling = 1_500_000
    solutions = collections.defaultdict(set)
    for m in range(2, int(math.sqrt(ceiling))):
        for n in range(1, m):
            a = m**2 - n**2
            b = 2 * m * n
            c = m**2 + n**2
            length = a + b + c
            if length > ceiling:
                break
            for k in itertools.count(1):
                if k * length > ceiling:
                    break
                solutions[k * length].add((min(k * a, k * b), max(k * a, k * b), k * c))
    return sum(len(elements) == 1 for elements in solutions.values())


if __name__ == "__main__":
    import runner

    runner.run(globals())
