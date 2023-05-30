import math


def rectangles_in_area(height: int, width: int) -> int:
    result = 0
    for h in range(1, height + 1):
        for w in range(1, width + 1):
            result += (height - h + 1) * (width - w + 1)
    return result


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
