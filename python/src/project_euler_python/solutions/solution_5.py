from ..primes import get_prime_factors


def merge_factors(factors_1: dict[int, int], factors_2: dict[int, int]):
    for factor, count_2 in factors_2.items():
        count_1 = factors_1.get(factor, 0)
        factors_1[factor] = max(count_1, count_2)


def solution_factor_dicts() -> int:
    factors = {}
    for i in range(1, 21):
        new_factors = get_prime_factors(i)
        merge_factors(factors, new_factors)
    result = 1
    for factor, count in factors.items():
        result *= factor**count
    return result
