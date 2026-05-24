import itertools

from ..primes import prime_sieve, PrimeList


def solution_generator() -> int:
    return sum(itertools.takewhile(lambda prime: prime < 2_000_000, PrimeList()))


def solution_sieve() -> int:
    return sum(prime_sieve(2_000_000))
