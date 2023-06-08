import functools
import itertools
from typing import Optional

from tqdm import tqdm

from solution_66 import is_square


def integer_root(radicand: int) -> Optional[int]:
    result = radicand
    while True:
        new = (result + radicand // result) // 2
        if result == new:
            if result**2 == radicand:
                return result
            else:
                return None
        elif new > result:
            return None
        else:
            result = new


def shortest_path_is_integer(a, b_plus_c) -> bool:
    return is_square(a**2 + b_plus_c**2)


def multiplicity(a: int, b_plus_c: int) -> int:
    if b_plus_c <= a + 1:
        return b_plus_c // 2
    else:
        return (2 * a - b_plus_c + 2) // 2


def solution() -> int:
    ceiling = 1_000_000
    result = 0
    for a in itertools.count(1):
        for b_plus_c in range(1, 2 * a + 1):
            if shortest_path_is_integer(a, b_plus_c):
                result += multiplicity(a, b_plus_c)
                if result > ceiling:
                    return a


if __name__ == "__main__":
    import runner

    runner.run(globals())
