import functools
import itertools

from tqdm import tqdm
from problem_76 import partitions


@functools.cache
def partitions(number: int) -> int:
    if number == 0:
        return 1
    elif number < 0:
        return 0
    else:
        result = 0
        for k in itertools.count(1):
            part_1 = partitions(number - k * (3 * k - 1) // 2)
            part_2 = partitions(number - (-k) * (3 * (-k) - 1) // 2)
            sign = +1 if k % 2 == 1 else -1
            result += sign * (part_1 + part_2)
            if not part_1 and not part_2:
                break
        return result


def solution() -> int:
    for n in tqdm(itertools.count(5)):
        if partitions(n) % 1_000_000 == 0:
            return n


if __name__ == "__main__":
    import runner

    runner.run(globals())
