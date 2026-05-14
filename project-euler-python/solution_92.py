from collections.abc import Iterator
import functools
import math


def get_digit_square_sum(number: int) -> int:
    digits = number
    number = 0
    while digits:
        number += (digits % 10) ** 2
        digits //= 10
    return number


@functools.cache
def get_terminator(number: int) -> int:
    if number == 0:
        return 0
    if number == 1 or number == 89:
        return number
    else:
        return get_terminator(get_digit_square_sum(number))


def iter_digits(counts: list[int], used: int) -> Iterator[list[int]]:
    if len(counts) == 10:
        yield counts
    else:
        lower = 0 if len(counts) < 9 else 7 - used
        upper = 8 - used
        for k in range(lower, upper):
            counts.append(k)
            yield from iter_digits(counts, used + k)
            counts.pop()


def solution() -> int:
    terminates_in_89 = 0
    for ks in iter_digits([], 0):
        multiplicity = math.factorial(7)
        for kd in ks:
            multiplicity //= math.factorial(kd)
        terminator = get_terminator(sum(kd * d**2 for d, kd in enumerate(ks)))
        if terminator == 89:
            terminates_in_89 += multiplicity

    return terminates_in_89


if __name__ == "__main__":
    import runner

    runner.run(globals())
