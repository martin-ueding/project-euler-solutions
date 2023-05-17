import collections
import itertools
from typing import Iterator


def iter_cubes() -> Iterator[int]:
    for i in itertools.count(1):
        yield i**3


def sort_digits(number: int) -> list[int]:
    return sorted(str(number))


def solution() -> int:
    digit_dict = collections.defaultdict(list)
    for cube in iter_cubes():
        digits = tuple(sort_digits(cube))
        digit_dict[digits].append(cube)
        if len(digit_dict[digits]) == 5:
            return min(digit_dict[digits])


if __name__ == "__main__":
    import runner

    runner.run(globals())
