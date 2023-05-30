from solution_3 import prime_generator
from solution_7 import prime_sieve


def _solution_generator() -> int:
    prime_sum = 0
    for prime in prime_generator():
        if prime < 2_000_000:
            prime_sum += prime
    return prime_sum


def solution_sieve() -> int:
    primes = prime_sieve(2_000_000)
    return sum(primes)


if __name__ == "__main__":
    import runner

    runner.run(globals())
