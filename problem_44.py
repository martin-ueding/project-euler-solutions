import itertools
import math
from typing import Iterator


def iter_pentagonal_numbers() -> Iterator[int]:
    p = 1
    d = 4
    while True:
        yield p
        p += d
        d += 3


def is_pentagonal(number: int) -> bool:
    s = int(round(math.sqrt(1 + 24 * number)))
    return s**2 == 1 + 24 * number and (1 + s) % 6 == 0


def test_is_pentagonal() -> None:
    assert is_pentagonal(1)
    assert is_pentagonal(5)
    assert is_pentagonal(12)
    assert is_pentagonal(22)
    assert not is_pentagonal(2)
    assert not is_pentagonal(3)
    assert not is_pentagonal(4)

    for p in itertools.islice(iter_pentagonal_numbers(), 1_000_000):
        assert is_pentagonal(p)


def _solution_diff() -> int:
    for p_diff in iter_pentagonal_numbers():
        for p_lower, p_lower_next in itertools.pairwise(iter_pentagonal_numbers()):
            upper = p_lower + p_diff
            if upper < p_lower_next:
                print(p_diff, p_lower)
                break
            sum_ = p_lower + upper
            if is_pentagonal(upper) and is_pentagonal(sum_):
                return p_diff


def solution_sum_lower() -> int:
    for p_sum in iter_pentagonal_numbers():
        for p_lower in iter_pentagonal_numbers():
            upper = p_sum - p_lower
            if upper < p_lower:
                break
            diff = upper - p_lower
            if is_pentagonal(upper) and is_pentagonal(diff):
                return diff


if __name__ == "__main__":
    import runner

    runner.run(globals())
