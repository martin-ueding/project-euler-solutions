import itertools

from solution_40 import generate_digits


def test_generate_digits() -> None:
    s = "".join(itertools.islice(generate_digits(), 20))
    assert s == "12345678910111213141"
