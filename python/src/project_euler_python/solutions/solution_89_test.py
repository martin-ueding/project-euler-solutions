from python.src.project_euler_python.solutions.solution_89 import (
    parse_roman_numeral,
    write_as_roman,
)


def test_parse_roman_numeral() -> None:
    cases = {
        "IV": 4,
        "IIII": 4,
        "XIX": 19,
        "XXXXIIIIIIIII": 49,
        "XXXXVIIII": 49,
        "XXXXIX": 49,
        "XLIX": 49,
    }
    for roman, arabic in cases.items():
        assert parse_roman_numeral(roman) == arabic


def test_two_way() -> None:
    for i in range(5_000):
        roman = write_as_roman(i)
        assert parse_roman_numeral(roman) == i
