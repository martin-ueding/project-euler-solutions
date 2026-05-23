import itertools

from ..primes import prime_sieve

from ..primes import PrimeList


def solution_generator() -> int:
    return list(itertools.islice(PrimeList(), 10000, 10001))[0]


def solution_sieve() -> int:
    primes = prime_sieve(110_000)
    return primes[10000]


if __name__ == "__main__":
    sixth = list(itertools.islice(PrimeList(), 5, 6))[0]
    assert sixth == 13

    import python.src.project_euler_python.runner as runner

    runner.run(globals())
