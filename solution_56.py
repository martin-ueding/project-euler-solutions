def digit_sum(number: int) -> int:
    return sum(map(int, str(number)))


def solution_naive() -> int:
    return max(digit_sum(a**b) for a in range(1, 100) for b in range(1, 100))


if __name__ == "__main__":
    import runner

    runner.run(globals())
