import itertools
import math
from typing import Iterator

from tqdm import tqdm

from solution_7 import prime_sieve


def powers(primes: list[int], exponent: int) -> Iterator[int]:
    for base in primes:
        yield base**exponent


def solution() -> int:
    ceiling = 50_000_000
    primes = prime_sieve(int(math.sqrt(ceiling)))
    numbers = set()
    for square in tqdm(powers(primes, 2)):
        if square > ceiling:
            break
        for cube in powers(primes, 3):
            if square + cube > ceiling:
                break
            for quadruple in powers(primes, 4):
                number = square + cube + quadruple
                if number >= ceiling:
                    break
                numbers.add(number)
    return len(numbers)


if __name__ == "__main__":
    import runner

    runner.run(globals())
