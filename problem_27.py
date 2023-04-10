import itertools

from problem_3 import prime_generator


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


def test_prime_set() -> None:
    prime_set = PrimeSet()
    assert prime_set.contains(7)
    assert not prime_set.contains(9)


def solution() -> int:
    n_max = 0
    best = (None, None)
    prime_set = PrimeSet()
    for a in range(-999, 1000):
        for b in range(2, 1001):
            n_end = 0
            for n in itertools.count():
                candidate = n**2 + a * n + b
                if candidate <= 0:
                    break
                if not prime_set.contains(candidate):
                    break
                n_end += 1
            if n_end > n_max:
                best = (a, b)
                n_max = n_end
    return best[0] * best[1]


if __name__ == "__main__":
    import runner

    runner.run(globals())
