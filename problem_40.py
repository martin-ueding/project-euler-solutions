import itertools
from typing import Iterator


def generate_digits() -> Iterator[str]:
    for number in itertools.count(1):
        s = str(number)
        yield from s


def test_generate_digits() -> None:
    s = "".join(itertools.islice(generate_digits(), 20))
    assert s == "12345678910111213141"


def solution() -> int:
    result = 1
    power_of_ten = 1
    for i, digit in enumerate(generate_digits(), start=1):
        if i == power_of_ten:
            result *= int(digit)
            power_of_ten *= 10
            if power_of_ten > 1_000_000:
                break
    return result


if __name__ == "__main__":
    import runner

    runner.run(globals())
