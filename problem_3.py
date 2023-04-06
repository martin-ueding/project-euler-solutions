import itertools


number = 600851475143


def prime_generator() -> int:
    primes = []
    for candidate in itertools.count(2):
        for prime in primes:
            if candidate % prime == 0:
                break
        else:
            yield candidate
            primes.append(candidate)


def _solution_naive() -> int:
    factor = None
    for prime in prime_generator():
        if number % prime == 0:
            factor = prime
        if prime > number:
            break
    return factor


def solution_reducing() -> int:
    remainder = number
    last_factor = None
    for prime in prime_generator():
        while remainder % prime == 0:
            last_factor = prime
            remainder /= prime
        if remainder == 1:
            break
    return last_factor


if __name__ == "__main__":
    import runner
    runner.run(globals())