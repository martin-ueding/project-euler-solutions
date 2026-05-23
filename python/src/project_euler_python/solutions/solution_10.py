import itertools

from project_euler_python.primes import prime_sieve, PrimeList


def solution_generator() -> int:
    return sum(itertools.takewhile(lambda prime: prime < 2_000_000, PrimeList()))


def solution_sieve() -> int:
    primes = prime_sieve(2_000_000)
    return sum(primes)

