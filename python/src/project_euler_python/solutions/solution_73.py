from project_euler_python.specnum import (
    greatest_common_denominator,
)
from python.src.project_euler_python.solutions.solution_72 import farey_sequence


def solution_count_reduced() -> int:
    result = 0
    for denominator in range(1, 12_000 + 1):
        for numerator in range(1, denominator):
            if denominator < numerator * 3 and 2 * numerator < denominator:
                if greatest_common_denominator(numerator, denominator) == 1:
                    result += 1
    return result


def solution_farey() -> int:
    result = 0
    for numerator, denominator in farey_sequence(12_000):
        if denominator < numerator * 3 and 2 * numerator < denominator:
            result += 1
    return result


if __name__ == "__main__":
    import python.src.project_euler_python.runner as runner

    runner.run(globals())
