import functools


values = list(range(4, 0, -1))


@functools.cache
def partitions(number: int, top: int) -> int:
    if number == 0:
        return []
    if number == 1:
        return [(1,)]
    else:
        result = [
            (x,) + p
            for x in range(1, top + 1)
            for p in partitions(number - x, min(number - x, x))
        ]
        if number <= top:
            result += [(number,)]
        return result


def num_partitions(number: int) -> int:
    ps = partitions(number, number)
    ps.sort()
    print(number, ps)
    return len(ps)


def solution() -> int:
    return num_partitions(100) - 1


if __name__ == "__main__":
    import runner

    runner.run(globals())
