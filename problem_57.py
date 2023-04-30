import itertools
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


def solution() -> int:
    terms_of_interest = list(
        filter(
            lambda term: len(str(term[0])) > len(str(term[1])),
            itertools.islice(square_root_series(), 1000),
        )
    )
    return len(terms_of_interest)


if __name__ == "__main__":
    import runner

    runner.run(globals())
