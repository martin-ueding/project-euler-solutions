import itertools
from typing import Callable, Iterator

from solution_33 import greatest_common_denominator
from solution_56 import digit_sum


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


def convergents_series(coefficients: Iterator[int]) -> Iterator[int]:
    coefficients_so_far = []
    for coefficient in coefficients:
        coefficients_so_far.append(coefficient)
        yield convergent_from_sequence(coefficients_so_far)


def solution() -> int:
    fractions = list(
        itertools.islice(convergents_series(continued_fraction_e()), 99, 100)
    )
    return digit_sum(fractions[0][0])


if __name__ == "__main__":
    import runner

    runner.run(globals())
