import itertools
from problem_42 import is_triangle_number
from problem_44 import is_pentagonal


def solution() -> int:
    for n in itertools.count(144):
        hex_n = n * (2 * n - 1)
        if is_pentagonal(hex_n) and is_triangle_number(hex_n):
            return hex_n


if __name__ == "__main__":
    import runner

    runner.run(globals())
