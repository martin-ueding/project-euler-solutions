data = """
75
95 64
17 47 82
18 35 87 10
20 04 82 47 65
19 01 23 75 03 34
88 02 77 73 07 63 67
99 65 04 28 06 16 70 92
41 41 26 56 83 40 80 70 33
41 48 72 33 47 32 37 16 94 29
53 71 44 65 25 43 91 52 97 51 14
70 11 33 28 77 73 17 78 39 68 17 57
91 71 52 38 17 14 91 43 58 50 27 29 48
63 66 04 68 89 53 67 30 73 16 69 87 40 31
04 62 98 27 23 09 70 98 73 93 38 53 60 04 23
"""


def parse_triangle() -> list[list[int]]:
    return [[int(word) for word in line.split()] for line in data.strip().split("\n")]


def traverse(triangle: list[list[int]], row_i: int, col_i: int) -> int:
    this = triangle[row_i][col_i]
    if row_i == len(triangle) - 1:
        return this
    else:
        return this + max(
            traverse(triangle, row_i + 1, col_i),
            traverse(triangle, row_i + 1, col_i + 1),
        )


def solution_brute_force() -> int:
    triangle = parse_triangle()
    return traverse(triangle, 0, 0)


def solution_bottom_up() -> int:
    triangle = parse_triangle()
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
