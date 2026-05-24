import itertools

from ..primes import prime_sieve
from ..primes import PrimeList


def solution_generator() -> int:
    return list(itertools.islice(PrimeList(), 10000, 10001))[0]


def solution_sieve() -> int:
    primes = prime_sieve(110_000)
    return primes[10000]
