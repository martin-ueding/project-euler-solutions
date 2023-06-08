import functools
import itertools
import math
from typing import Optional

from tqdm import tqdm


def path_length(a: int, b: int, c: int) -> float:
    return math.sqrt(((a * b) / (b + c)) ** 2 + b**2) + math.sqrt(
        (a - (a * b) / (b + c)) ** 2 + c**2
    )


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


def shortest_path_is_integer(a, b, c) -> bool:
    return shortest_path_cache(a, b + c)


@functools.cache
def shortest_path_cache(a, bc) -> bool:
    return integer_root(a**2 + bc**2) is not None


def solution() -> int:
    ceiling = 1_000_000
    result = 0
    with tqdm(total=ceiling) as pbar:
        for a in itertools.count(1):
            for b in range(1, a + 1):
                for c in range(1, b + 1):
                    if shortest_path_is_integer(a, b, c):
                        result += 1
                        pbar.update(1)
                        if result > ceiling:
                            return a


if __name__ == "__main__":
    import runner

    runner.run(globals())
