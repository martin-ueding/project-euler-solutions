import functools
import itertools


@functools.cache
def factorial(n: int) -> int:
    return functools.reduce(lambda a, b: a * b, range(1, n + 1), 1)


def digit_factorial_sum(n: int) -> int:
    return sum(factorial(int(c)) for c in str(n))


def upper_limit() -> int:
    for num_digits in itertools.count(1):
        if factorial(9) * num_digits < 10**num_digits:
            return factorial(9) * num_digits + 1


def solution() -> int:
    return sum(
        number
        for number in range(3, upper_limit())
        if number == digit_factorial_sum(number)
    )


if __name__ == "__main__":
    import runner

    runner.run(globals())
