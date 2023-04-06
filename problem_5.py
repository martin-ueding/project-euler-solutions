import collections
from problem_3 import prime_generator


def get_prime_factors(number: int) -> dict[int, int]:
    factors = collections.defaultdict(lambda: 0)
    for prime in prime_generator():
        while number % prime == 0:
            factors[prime] += 1
            number /= prime
        if number == 1:
            break
    return factors


def merge_factors(factors_1: dict[int, int], factors_2: dict[int, int]):
    for factor, count_2 in factors_2.items():
        count_1 = factors_1.get(factor, 0)
        factors_1[factor] = max(count_1, count_2)


def solution_factor_dicts() -> None:
    factors = {}
    for i in range(1, 21):
        new_factors = get_prime_factors(i)
        merge_factors(factors, new_factors)
    result = 1
    for factor, count in factors.items():
        result *= factor**count
    return result


if __name__ == "__main__":
    import runner

    runner.run(globals())
