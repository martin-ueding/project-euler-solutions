import math
from typing import Iterator

from problem_64 import expand_root
from problem_65 import convergents_series


def is_square(number: int) -> bool:
    floor = int(math.sqrt(number))
    return floor**2 == number


def square_root_fraction_expansion(number: int) -> Iterator[int]:
    prefix, period = expand_root(number)
    yield from prefix
    while True:
        yield from period


def square_root_convergents(number: int) -> Iterator[tuple[int, int]]:
    yield from convergents_series(square_root_fraction_expansion(number))


def minimal_solution(d: int) -> int:
    for x, y in square_root_convergents(d):
        if x == 1:
            continue
        if x**2 - d * y**2 == 1:
            return x


def solution() -> int:
    max_x = 0
    max_d = 0
    for d in range(1, 1001):
        if is_square(d):
            continue
        x = minimal_solution(d)
        if x > max_x:
            max_x = x
            max_d = d
    return max_d


if __name__ == "__main__":
    import runner

    runner.run(globals())
