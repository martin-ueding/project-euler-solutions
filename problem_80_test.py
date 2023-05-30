from problem_80 import sqrt_digits
from problem_56 import digit_sum


def test_sqrt_digits() -> None:
    sqrt_2 = sqrt_digits(2)
    assert digit_sum(sqrt_2[:100]) == 475
