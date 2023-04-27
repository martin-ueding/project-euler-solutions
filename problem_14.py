def collatz_length(start: int) -> int:
    steps = 0
    while start != 1:
        if start % 2 == 0:
            start //= 2
        else:
            start = 3 * start + 1
        steps += 1
    return steps


def _solution_naive() -> int:
    result = 0
    max_steps = 0
    for start in range(1, 1_000_000):
        steps = collatz_length(start)
        if steps > max_steps:
            max_steps = steps
            result = start
    return result


class CollatzCache:
    def __init__(self) -> None:
        self._cache = {}

    def compute_length(self, start: int) -> int:
        steps = 0
        number = start
        trajectory = [start]
        while number != 1:
            if number in self._cache:
                self._insert_result(trajectory, self._cache[number])
                return self._cache[start]

            if number % 2 == 0:
                number //= 2
            else:
                number = 3 * number + 1
            trajectory.append(number)
            steps += 1

        self._insert_result(trajectory, 0)
        return steps

    def _insert_result(self, trajectory: list[int], remaining_steps: int) -> None:
        for i, start in enumerate(trajectory):
            self._cache[start] = len(trajectory) - i + remaining_steps


def solution_cached() -> int:
    collatz_cache = CollatzCache()
    result = 0
    max_steps = 0
    for start in range(1, 1_000_000):
        steps = collatz_cache.compute_length(start)
        if steps > max_steps:
            max_steps = steps
            result = start
    return result


def recursive_collatz_length(start: int) -> int:
    if start == 1:
        return 0
    elif start % 2 == 0:
        return 1 + recursive_collatz_length(start // 2)
    else:
        return 1 + recursive_collatz_length(3 * start + 1)


def _solution_recursive() -> int:
    result = 0
    max_steps = 0
    for start in range(1, 1_000_000):
        steps = collatz_length(start)
        if steps > max_steps:
            max_steps = steps
            result = start
    return result


class RecursiveCollatzCache:
    def __init__(self) -> None:
        self._cache = {}

    def compute_length(self, start: int) -> int:
        if start in self._cache:
            return self._cache[start]

        if start == 1:
            result = 0
        elif start % 2 == 0:
            result = 1 + self.compute_length(start // 2)
        else:
            result = 1 + self.compute_length(3 * start + 1)

        self._cache[start] = result
        return result


def solution_recursive_with_cache() -> int:
    cache = RecursiveCollatzCache()
    result = 0
    max_steps = 0
    for start in range(1, 1_000_000):
        steps = cache.compute_length(start)
        if steps > max_steps:
            max_steps = steps
            result = start
    return result


if __name__ == "__main__":
    import runner

    runner.run(globals())
