import functools


@functools.cache
def factorial(number: int) -> int:
    result = 1
    for k in range(2, number + 1):
        result *= k
    return result


@functools.cache
def factorial_digit_sum(number: int) -> int:
    return sum(factorial(int(digit)) for digit in str(number))


def solution() -> int:
    result = 0
    for number in range(1_000_000):
        steps = [number]
        for iteration in range(60):
            new_number = factorial_digit_sum(steps[-1])
            if new_number in steps:
                if iteration == 59:
                    result += 1
                break
            steps.append(new_number)
    return result


if __name__ == "__main__":
    import runner

    runner.run(globals())
