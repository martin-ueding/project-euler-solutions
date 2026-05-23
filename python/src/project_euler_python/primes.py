from collections.abc import Iterator
import itertools


def prime_generator(_primes=[]) -> Iterator[int]:
    yield from _primes
    start = 2 if not _primes else _primes[-1] + 1
    for candidate in itertools.count(start):
        is_prime = True
        for prime in _primes:
            if prime * prime > candidate:
                break
            if candidate % prime == 0:
                is_prime = False
                break
        if is_prime:
            yield candidate
            _primes.append(candidate)



class PrimeList:
    def __init__(self) -> None:
        self._primes = []

    def __iter__(self) -> Iterator[int]:
        yield from self._primes
        start = 2 if not self._primes else self._primes[-1] + 1
        for candidate in itertools.count(start):
            is_prime = True
            for prime in self._primes:
                if prime * prime > candidate:
                    break
                if candidate % prime == 0:
                    is_prime = False
                    break
            if is_prime:
                yield candidate
                self._primes.append(candidate)