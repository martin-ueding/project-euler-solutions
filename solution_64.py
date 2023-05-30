import math
from typing import Iterator

from solution_33 import greatest_common_denominator


def expand_root(number: int) -> tuple[list[int], list[int]]:
    floor = int(math.sqrt(number))
    if floor**2 == number:
        return [floor], []
    results = [floor]
    states = [(1, floor)]
    c = floor
    b = 1
    while True:
        # print()
        assert c > 0
        # print(f"{b}/(sqrt({number}) - {c})")
        d = number - c**2
        gcd = greatest_common_denominator(b, d)
        # print(f"{b} (sqrt({number}) + {c})/{d}")
        b //= gcd
        d //= gcd
        # print(f"{b} (sqrt({number}) + {c})/{d}")
        split = (floor + c) // d
        a = split * b
        c -= split * d
        # print(f"{a} + {b} (sqrt({number}) + {c})/{d}")
        c = -c
        b = d
        state = (b, c)
        results.append(a)
        if state in states:
            break
        states.append(state)
    i = states.index(state) + 1
    return results[:i], results[i:]


def solution() -> int:
    result = 0
    for number in range(2, 10_000):
        beginning, period = expand_root(number)
        if len(period) % 2 == 1:
            result += 1
    return result


if __name__ == "__main__":
    import runner

    runner.run(globals())
