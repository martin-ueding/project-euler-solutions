import bisect
import functools


@functools.cache
def rectangles_along_axis(length: int) -> int:
    return sum((length - l + 1) for l in range(1, length + 1))


def solution() -> int:
    goal = 2_000_000
    results = {}
    numbers = [rectangles_along_axis(length) for length in range(1, 100)]
    for width, num_width in enumerate(numbers, 1):
        j = bisect.bisect_left(numbers, goal // num_width)
        for height in [j + 1, j + 2]:
            if 0 <= height - 1 < len(numbers):
                num_height = numbers[height - 1]
                num = num_height * num_width
                results[abs(goal - num)] = height * width
    return min(results.items())[1]


if __name__ == "__main__":
    import runner

    runner.run(globals())
