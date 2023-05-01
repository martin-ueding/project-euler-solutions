values = list(range(4, 0, -1))


def count_partitions(used: tuple, remainder: int) -> int:
    if len(used) == len(values) - 1:
        return 1

    current_value = values[len(used)]

    return sum(
        count_partitions(used + (usage,), remainder - current_value * usage)
        for usage in range(remainder // current_value + 1)
    )


def solution() -> int:
    return count_partitions((), 5)


if __name__ == "__main__":
    import runner

    runner.run(globals())
