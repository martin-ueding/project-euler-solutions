def get_num_solutions(perimeter: int) -> int:
    num_solutions = 0
    for a in range(1, perimeter // 3):
        for b in range(a, perimeter - 2 * a + 1):
            c = perimeter - a - b
            if a**2 + b**2 == c**2:
                num_solutions += 1
    return num_solutions


def test_get_num_solutions() -> None:
    assert get_num_solutions(120) == 3


def solution() -> int:
    return max(
        (get_num_solutions(perimeter), perimeter) for perimeter in range(2, 1000, 2)
    )[1]


if __name__ == "__main__":
    import runner

    runner.run(globals())
