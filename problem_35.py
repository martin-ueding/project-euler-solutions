import itertools
from typing import Iterator

from tqdm import tqdm

from problem_3 import prime_generator
from problem_7 import prime_sieve


def iter_cycles(n: int) -> Iterator[int]:
    digits = str(n)
    for _ in range(len(digits)):
        yield int(digits)
        digits = digits[1:] + digits[0]


def solution() -> int:
    num_cyclic_primes = 0
    primes = prime_sieve(1_000_000)
    for prime in primes:
        if set(str(prime)) & {"0", "2", "4", "5", "6", "8"}:
            continue
        for cycle in iter_cycles(prime):
            if cycle not in primes:
                break
        else:
            num_cyclic_primes += 1
    return num_cyclic_primes


if __name__ == "__main__":
    import runner

    runner.run(globals())
