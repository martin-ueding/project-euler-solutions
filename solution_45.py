import itertools

from solution_44 import is_pentagonal


def solution() -> int:
    for n in itertools.count(144):
        hex_n = n * (2 * n - 1)
        if is_pentagonal(hex_n):
            return hex_n


if __name__ == "__main__":
    import runner

    runner.run(globals())
