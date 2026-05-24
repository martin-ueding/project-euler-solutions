import itertools

from ..primes import PrimeSet


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
