import math


def get_pairs() -> list[tuple[int, int]]:
    with open("data/p099_base_exp.txt") as f:
        return [tuple(map(int, line.split(","))) for line in f]


def solution_logarithm() -> int:
    pairs = get_pairs()
    numbers = [exponent * math.log(base) for base, exponent in pairs]
    argmax = sorted(enumerate(numbers, 1), key=lambda t: t[1])[-1][0]
    return argmax


if __name__ == "__main__":
    import runner

    runner.run(globals())
