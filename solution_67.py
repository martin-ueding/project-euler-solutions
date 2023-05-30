def load_triangle() -> list[list[int]]:
    with open("data/p067_triangle.txt") as f:
        data = f.read()
    return [[int(word) for word in line.split()] for line in data.strip().split("\n")]


def solution_bottom_up() -> int:
    triangle = load_triangle()
    for row_i in reversed(range(len(triangle) - 1)):
        row = triangle[row_i]
        for col_i in range(len(row)):
            row[col_i] += max(
                triangle[row_i + 1][col_i], triangle[row_i + 1][col_i + 1]
            )
    return triangle[0][0]


if __name__ == "__main__":
    import runner

    runner.run(globals())
