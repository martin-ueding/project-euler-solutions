import itertools
from typing import Iterator

from problem_27 import PrimeSet
from problem_3 import prime_generator
from problem_7 import prime_sieve


def iter_diagonals() -> Iterator[int]:
    number = 1
    for winding in itertools.count(1):
        elements = []
        for corner in range(4):
            number += 2 * winding
            elements.append(number)
        yield elements


def is_prime_accelerated(number: int) -> bool:
    for divisor in prime_generator():
        if number % divisor == 0:
            return False
        if divisor * divisor > number:
            return True


def solution() -> int:
    num_primes = 0
    num_total = 1
    for winding, numbers in enumerate(iter_diagonals(), start=1):
        for number in numbers:
            if is_prime_accelerated(number):
                num_primes += 1
        num_total += 4
        if 10 * num_primes < num_total:
            return 2 * winding + 1


if __name__ == "__main__":
    import runner

    runner.run(globals())
