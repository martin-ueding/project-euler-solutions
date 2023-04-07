def solution_naive() -> int:
    sum_of_multiples = 0
    for i in range(1000):
        if i % 3 == 0 or i % 5 == 0:
            sum_of_multiples += i
    return sum_of_multiples


def solution_set() -> int:
    multiples_of_3 = {i * 3 for i in range(1, 999 // 3 + 1)}
    multiples_of_5 = {i * 5 for i in range(1, 999 // 5 + 1)}
    all_multiples = multiples_of_3 | multiples_of_5
    return sum(all_multiples)


def sum_of_natural_numbers(end: int, step: int) -> int:
    count = end // step + 1
    return count * (count - 1) * step // 2


def solution_closed_form() -> int:
    return (
        sum_of_natural_numbers(999, 3)
        + sum_of_natural_numbers(999, 5)
        - sum_of_natural_numbers(999, 15)
    )


if __name__ == "__main__":
    import runner

    runner.run(globals())
