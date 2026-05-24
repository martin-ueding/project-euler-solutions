from typing import Iterator


def fibonacci_generator() -> Iterator[int]:
    yield 1
    yield 2
    previous = 1
    current = 2
    while True:
        previous, current = current, current + previous
        yield current


def greatest_common_denominator(a: int, b: int) -> int:
    while b != 0:
        b, a = a % b, b
    return a


def is_palindrome(n: int) -> bool:
    s = str(n)
    return s == s[::-1]
