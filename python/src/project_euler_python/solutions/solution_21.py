from ..primes import get_all_divisors


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
