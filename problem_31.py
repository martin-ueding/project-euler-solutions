values = [200, 100, 50, 20, 10, 5, 2, 1]


def count_partitions(used: tuple, remainder: int) -> int:
    if len(used) == len(values):
        if remainder == 0:
            return 1
        else:
            return 0

    current_value = values[len(used)]

    return sum(
        count_partitions(used + (usage,), remainder - current_value * usage)
        for usage in range(remainder // current_value + 1)
    )


def solution() -> int:
    return count_partitions((), 200)


if __name__ == "__main__":
    import runner

    runner.run(globals())
