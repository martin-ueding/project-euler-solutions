import itertools
from tqdm import tqdm

values = [200, 100, 50, 20, 10, 5, 2, 1]


def _solution_brute_force() -> int:
    num_partitions = 0
    ranges = [range(200 // value + 1) for value in values]
    print(ranges)
    for counts in tqdm(itertools.product(*ranges)):
        total_value = sum(value * count for value, count in zip(values, counts))
        if total_value == 200:
            num_partitions += 1
    return num_partitions


def count_partitions(used: tuple, remainder: int) -> int:
    if len(used) == len(values) - 1:
        return 1

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
