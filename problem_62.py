import collections
import itertools
from typing import Iterator

from tqdm import tqdm


def iter_cubes() -> Iterator[int]:
    for i in itertools.count(1):
        yield i**3


class CubeSet:
    def __init__(self) -> None:
        self._cubes: list[int] = [0]
        self._iterator = iter_cubes()

    def is_cube(self, number: int) -> bool:
        while self._cubes[-1] < number:
            self._cubes.append(next(self._iterator))
        return number in self._cubes


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
