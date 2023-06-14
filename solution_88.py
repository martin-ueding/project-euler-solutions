import functools
from typing import Iterator


def iter_partitions(number: int) -> Iterator[list[int]]:
    yield from partitions(number, number)


def partitions(number: int, top: int) -> int:
    if number == 0:
        yield []
    if number == 1:
        yield [1]
    else:
        for x in range(1, top + 1):
            for result in partitions(number - x, min(number - x, x)):
                yield [x] + result


def solution() -> int:
    minimum_sum_product_numbers: dict[int, int] = {}
    for number in range(20):
        for partition in iter_partitions(number):
            if len(partition) > 12_000:
                continue
            if not len(partition) in minimum_sum_product_numbers:
                product = functools.reduce(lambda a, b: a * b, partition, 1)
                if product == number:
                    minimum_sum_product_numbers[len(partition)] = number
    return minimum_sum_product_numbers


if __name__ == "__main__":
    import runner

    runner.run(globals())
