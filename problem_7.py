import itertools

from problem_3 import prime_generator


def solution_generator() -> int:
    return list(itertools.islice(prime_generator(), 10000, 10001))[0]


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


def solution_sieve() -> int:
    primes = prime_sieve(110_000)
    return primes[10000]


if __name__ == "__main__":
    sixth = list(itertools.islice(prime_generator(), 5, 6))[0]
    assert sixth == 13

    import runner

    runner.run(globals())
