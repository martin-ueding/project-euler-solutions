import math


def rectangles_along_axis(length: int) -> int:
    return sum((length - l + 1) for l in range(1, length + 1))


def rectangles_in_area(height: int, width: int) -> int:
    return rectangles_along_axis(height) * rectangles_along_axis(width)


def solution() -> int:
    goal = 2_000_000
    results = {}
    for width in range(1, 100):
        for height in range(1, 100):
            num = rectangles_in_area(height, width)
            if num > 2_100_000:
                break
            results[abs(goal - num)] = (height, width, height * width)
    return min(results.items())[1][2]


if __name__ == "__main__":
    import runner

    runner.run(globals())
