import functools
import itertools
from typing import Optional

from tqdm import tqdm



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


@functools.cache
def shortest_path_is_integer(a, b_plus_c) -> bool:
    return integer_root(a**2 + b_plus_c**2) is not None


def solution() -> int:
    ceiling = 1_000_000
    result = 0
    with tqdm(total=ceiling) as pbar:
        for a in itertools.count(1):
            for b_plus_c in range(2, 2*a + 1):
                if shortest_path_is_integer(a, b_plus_c):
                    num_b_c = b_plus_c // 2
                    result += num_b_c
                    pbar.update(num_b_c)
                    if result > ceiling:
                        return a


if __name__ == "__main__":
    import runner

    runner.run(globals())
