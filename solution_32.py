import itertools


def solution() -> int:
    results = set()
    for digits in itertools.permutations("123456789"):
        for sep_1 in range(2, 7):
            for sep_2 in range(sep_1 + 1, 8):
                factor_1 = int("".join(digits[:sep_1]))
                factor_2 = int("".join(digits[sep_1:sep_2]))
                product = int("".join(digits[sep_2:]))
                if factor_1 * factor_2 == product:
                    results.add(product)
    return sum(results)


if __name__ == "__main__":
    import runner

    runner.run(globals())
