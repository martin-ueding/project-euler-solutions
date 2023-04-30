def get_num_solutions(perimeter: int) -> int:
    num_solutions = 0
    for a in range(1, perimeter // 3):
        numerator = perimeter**2 - 2 * a * perimeter
        denominator = 2 * (perimeter - a)
        if numerator % denominator == 0:
            b = numerator // denominator
            if a <= b < 1000:
                num_solutions += 1
    return num_solutions


def solution() -> int:
    return max(
        (get_num_solutions(perimeter), perimeter) for perimeter in range(2, 1000, 2)
    )[1]


if __name__ == "__main__":
    import runner

    runner.run(globals())
