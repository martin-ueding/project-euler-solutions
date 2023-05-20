import functools
import itertools

from problem_3 import prime_generator
from problem_7 import prime_sieve
from problem_58 import is_prime_accelerated


def primes_up_to(number: int) -> int:
    for prime in prime_generator():
        if prime > number:
            break
        yield prime


@functools.cache
def partitions(number: int, top: int) -> int:
    if number == 0:
        return 0
    elif number == 1:
        return 0
    elif number == 2:
        return 1
    else:
        result = sum(
            partitions(number - x, min(number - x, x)) for x in primes_up_to(top)
        )
        if number <= top and is_prime_accelerated(number):
            result += 1
        return result


def solution() -> int:
    prime_generator.__defaults__ = (prime_sieve(10_000),)
    for i in itertools.count(10):
        if partitions(i, i) > 5_000:
            return i


if __name__ == "__main__":
    import runner

    runner.run(globals())
