import itertools
from typing import Iterator


def iter_diagonals() -> Iterator[int]:
    number = 1
    yield number
    for winding in itertools.count(1):
        for corner in range(4):
            number += 2 * winding
            yield number
