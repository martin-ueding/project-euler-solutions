import functools
import itertools

from typing import Iterator

from problem_5 import get_prime_factors


def triangle_number_generator() -> Iterator[int]:
    accumulator = 0
    for i in itertools.count(1):
        accumulator += i
        yield accumulator


def solution() -> int:
    for triangle_number in triangle_number_generator():
        prime_factors = get_prime_factors(triangle_number)
        num_divisors = functools.reduce(
            lambda a, b: a * b, map(lambda x: x + 1, prime_factors.values()), 1
        )
        if num_divisors > 500:
            print(triangle_number, dict(prime_factors.items()), num_divisors)
            return triangle_number


if __name__ == "__main__":
    import runner

    runner.run(globals())
