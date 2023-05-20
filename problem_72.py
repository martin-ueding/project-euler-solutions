from typing import Iterator
from problem_3 import prime_generator
from problem_5 import get_prime_factors
from problem_7 import prime_sieve


def farey_sequence(n: int) -> Iterator[tuple[int, int]]:
    # Adapted from https://en.wikipedia.org/wiki/Farey_sequence#Next_term
    (a, b, c, d) = (0, 1, 1, n)
    yield a, b
    while c <= n:
        k = (n + b) // d
        (a, b, c, d) = (c, d, k * c - a, k * d - b)
        yield a, b


def generator_len(sequence: Iterator) -> int:
    result = 0
    for elem in sequence:
        result += 1
    return result


def totient(n: int) -> int:
    prime_factors = get_prime_factors(n)
    result = 1
    for prime, multiplicity in prime_factors.items():
        result *= prime ** (multiplicity - 1) * (prime - 1)
    return result


def farey_length(n: int) -> int:
    result = 1
    for m in range(1, n + 1):
        result += totient(m)
    return result


def _solution() -> int:
    ceiling = 100000
    prime_generator.__default__ = (prime_sieve(ceiling),)
    return farey_length(ceiling) - 2


# With Möbius function


def generate_moebius_function(ceiling: int) -> list[int]:
    moebius = [None] * ceiling
    moebius[1] = 1
    for i in range(2, ceiling):
        # If we found a prime, set it to -1.
        if moebius[i] is None:
            moebius[i] = -1
            # Tick off the multiples.
            for k in range(2 * i, ceiling, i):
                if moebius[k] is None:
                    moebius[k] = -1
                else:
                    moebius[k] *= -1
            # Eliminate all squares.
            for k in range(i**2, ceiling, i**2):
                moebius[k] = 0
    return moebius


def summary_totient(ceiling: int) -> int:
    moebius = generate_moebius_function(ceiling + 1)
    result = 0
    for k in range(1, ceiling + 1):
        floor = ceiling // k
        result += moebius[k] * (floor) * (1 + floor)
    return result // 2


def solution() -> int:
    return summary_totient(1_000_000) - 1


if __name__ == "__main__":
    import runner

    runner.run(globals())
