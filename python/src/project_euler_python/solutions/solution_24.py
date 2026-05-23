import itertools


def solution_itertools() -> int:
    digits = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
    permutations = itertools.permutations(digits)
    millionth = next(itertools.islice(permutations, 999_999, 1_000_000))
    return int("".join(map(str, millionth)))


if __name__ == "__main__":
    import python.src.project_euler_python.runner as runner

    runner.run(globals())
