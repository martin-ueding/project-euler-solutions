from typing import Iterator

from problem_33 import greatest_common_denominator


def square_root_series() -> Iterator[tuple[int, int]]:
    n, d = 3, 2
    while True:
        yield n, d
        # Add 1.
        n += d
        # Invert.
        n, d = d, n
        # Add 1.
        n += d
        # Cancel.
        gcd = greatest_common_denominator(n, d)
        n //= gcd
        d //= gcd
