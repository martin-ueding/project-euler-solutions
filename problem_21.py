import functools
import itertools

from problem_5 import get_prime_factors


def get_all_divisors(number: int) -> set[int]:
    prime_factors = get_prime_factors(number)
    pairs = [
        (1, factor)
        for factor, multiplicity in prime_factors.items()
        for _ in range(multiplicity)
    ]
    if not pairs:
        return {1}
    divisors = {
        functools.reduce(lambda a, b: a * b, combination)
        for combination in itertools.product(*pairs)
    }
    divisors.remove(number)
    return divisors


def solution() -> int:
    divisor_sums = {
        number: sum(get_all_divisors(number)) for number in range(1, 10_000)
    }
    sum_of_amicable = 0
    for number, divisor_sum in divisor_sums.items():
        if divisor_sum >= 10_000:
            continue
        if divisor_sums[divisor_sum] == number and number != divisor_sum:
            sum_of_amicable += number
    return sum_of_amicable


if __name__ == "__main__":
    import runner

    runner.run(globals())
