import itertools


def find_limit() -> int:
    for num_digits in itertools.count(1):
        number = num_digits * 9**5
        if len(str(number)) <= num_digits:
            return number


def solution() -> int:
    accumulator = 0
    for number in range(2, find_limit()):
        digit_sum = sum(int(digit) ** 5 for digit in str(number))
        if number == digit_sum:
            accumulator += number
    return accumulator


if __name__ == "__main__":
    import runner

    runner.run(globals())
