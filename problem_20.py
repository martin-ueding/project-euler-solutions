import functools


def solution() -> int:
    return sum(map(int, str(functools.reduce(lambda a, b: a * b, range(1, 101)))))


if __name__ == "__main__":
    import runner

    runner.run(globals())
