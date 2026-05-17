import math

from solution_56 import digit_sum


def sqrt_digits(number: int) -> str:
    number *= 10**250
    root = number
    for i in range(100000):
        old = root
        root = (root + number // root) // 2
        if old == root:
            break
    return str(root)


def solution() -> int:
    result = 0
    for i in range(100):
        if math.floor(math.sqrt(i)) ** 2 != i:
            result += digit_sum(sqrt_digits(i)[:100])
    return result


if __name__ == "__main__":
    import runner

    runner.run(globals())
