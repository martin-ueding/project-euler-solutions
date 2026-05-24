import collections
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


def prime_sieve(end: int) -> list[int]:
    sieve = [True] * end
    sieve[0] = False
    sieve[1] = False
    for i in range(end):
        if sieve[i]:
            for factor in itertools.count(2):
                number = factor * i
                if number >= len(sieve):
                    break
                sieve[number] = False
    primes = [number for number, state in enumerate(sieve) if state]
    return primes


def get_prime_factors(number: int) -> dict[int, int]:
    factors = collections.defaultdict(lambda: 0)
    for prime in prime_generator():
        while number % prime == 0:
            factors[prime] += 1
            number /= prime
        if number == 1:
            break
    return factors


def get_all_divisors(number: int) -> set[int]:
    prime_factors = get_prime_factors(number)
    pairs = [
        (1, factor)
        for factor, multiplicity in prime_factors.items()
        for _ in range(multiplicity)
    ]
    if not pairs:
        return {1}
    divisors = {
        functools.reduce(lambda a, b: a * b, combination)
        for combination in itertools.product(*pairs)
    }
    divisors.remove(number)
    return divisors


class PrimeSet:
    def __init__(self) -> None:
        self._primes: set[int] = []
        self._largest: int = 0
        self._prime_iterator = prime_generator()

    def contains(self, candidate: int) -> bool:
        while self._largest < candidate:
            new_prime = next(self._prime_iterator)
            self._largest = new_prime
            self._primes.append(new_prime)
        return candidate in self._primes
