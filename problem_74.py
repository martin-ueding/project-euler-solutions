import functools

from tqdm import tqdm


@functools.cache
def factorial(number: int) -> int:
    result = 1
    for k in range(2, number + 1):
        result *= k
    return result


@functools.lru_cache(10_000_000)
def factorial_digit_sum(number: int) -> int:
    return sum(factorial(int(digit)) for digit in str(number))


def solution() -> int:
    result = 0
    for number in tqdm(range(1_000_000)):
        steps = [number]
        for iteration in range(65):
            new_number = factorial_digit_sum(steps[-1])
            if new_number in steps:
                break
            steps.append(new_number)
        if len(steps) == 60:
            result += 1
    return result


if __name__ == "__main__":
    import runner

    runner.run(globals())
