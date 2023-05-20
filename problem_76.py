import functools


values = list(range(4, 0, -1))


@functools.cache
def partitions(number: int, top: int) -> int:
    if number == 0:
        return 0
    if number == 1:
        return 1
    else:
        result = sum(
            partitions(number - x, min(number - x, x)) for x in range(1, top + 1)
        )
        if number <= top:
            result += 1
        return result


def num_partitions(number: int) -> int:
    return partitions(number, number)


def solution() -> int:
    return num_partitions(100) - 1


if __name__ == "__main__":
    import runner

    runner.run(globals())
