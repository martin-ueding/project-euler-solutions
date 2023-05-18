import itertools
from typing import Iterator

from problem_33 import greatest_common_denominator


def continued_fraction_sqrt_2() -> Iterator[int]:
    yield 1
    while True:
        yield 2


def continued_fraction_e() -> Iterator[int]:
    yield 2
    for k in itertools.count(1):
        yield 1
        yield 2 * k
        yield 1


def convergent_from_sequence(coefficients: list[int]) -> tuple[int, int]:
    denominoator = 1
    numerator = coefficients[-1]
    for coefficient in reversed(coefficients[:-1]):
        numerator, denominoator = denominoator, numerator
        numerator += coefficient * denominoator
    gcd = greatest_common_denominator(numerator, denominoator)
    numerator //= gcd
    denominoator //= gcd
    return numerator, denominoator
