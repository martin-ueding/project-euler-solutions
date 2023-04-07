import functools
import itertools

from typing import Iterator
from problem_3 import prime_generator

from problem_5 import get_prime_factors


def triangle_number_generator() -> Iterator[int]:
    accumulator = 0
    for i in itertools.count(1):
        accumulator += i
        yield accumulator


def _solution() -> int:
    prime_generator.__defaults__ = ([],)
    for triangle_number in triangle_number_generator():
        prime_factors = get_prime_factors(triangle_number)
        num_divisors = functools.reduce(
            lambda a, b: a * b, map(lambda x: x + 1, prime_factors.values()), 1
        )
        if num_divisors > 500:
            return triangle_number


def get_num_divisors(number: int) -> int:
    prime_factors = get_prime_factors(number)
    num_divisors = functools.reduce(
        lambda a, b: a * b, map(lambda x: x + 1, prime_factors.values()), 1
    )
    return num_divisors


def solution_coprime() -> int():
    prime_generator.__defaults__ = ([],)
    for n in itertools.count(1):
        triangle_number = n * (n + 1) // 2
        if triangle_number % 2 == 0:
            num_divisors = get_num_divisors(n // 2) * get_num_divisors(n + 1)
        else:
            num_divisors = get_num_divisors((n + 1) // 2) * get_num_divisors(n)
        if num_divisors > 500:
            return triangle_number


if __name__ == "__main__":
    import runner

    runner.run(globals())
