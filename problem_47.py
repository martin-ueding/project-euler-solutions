import functools
import itertools

from problem_5 import get_prime_factors


@functools.lru_cache(4)
def get_num_unique_factors(number: int) -> set[int]:
    return len(set(get_prime_factors(number).keys()))


def solution() -> int:
    for i in itertools.count(1):
        if all(get_num_unique_factors(i + n) == 4 for n in range(4)):
            return i


if __name__ == "__main__":
    import runner

    runner.run(globals())