import itertools

from ..specnum import fibonacci_generator


def fibonacci_recursive(n) -> int:
    if n == 0:
        return 1
    elif n == 1:
        return 2
    else:
        return fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2)


def solution_naive() -> int:
    fib_sum = 0
    for i in itertools.count():
        fib_i = fibonacci_recursive(i)
        if fib_i % 2 == 0:
            fib_sum += fib_i
        if fib_i > 4_000_000:
            break
    return fib_sum


def solution_generator() -> int:
    fib_sum = 0
    for fib_i in fibonacci_generator():
        if fib_i % 2 == 0:
            fib_sum += fib_i
        if fib_i > 4_000_000:
            break
    return fib_sum
