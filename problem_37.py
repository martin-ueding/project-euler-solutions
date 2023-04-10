from tqdm import tqdm
from problem_27 import PrimeSet
from problem_3 import prime_generator
from problem_7 import prime_sieve


def iter_truncations(number: int) -> int:
    s = str(number)
    for i in range(1, len(s)):
        yield int(s[i:])
        yield int(s[:-i])


def test_iter_truncations() -> None:
    truncations = list(iter_truncations(1234))
    assert truncations == [234, 123, 34, 12, 4, 1]


def solution() -> int:
    primes = prime_sieve(1_000_000)
    truncatable_primes = []
    for prime in primes:
        if prime < 10:
            continue
        if set(str(prime)[1:]) & {"0", "2", "4", "5", "6", "8"}:
            continue
        for truncation in iter_truncations(prime):
            if truncation not in primes:
                break
        else:
            truncatable_primes.append(prime)
            if len(truncatable_primes) == 11:
                break
    return sum(truncatable_primes)


if __name__ == "__main__":
    import runner

    runner.run(globals())
