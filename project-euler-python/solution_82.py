from solution_81 import read_matrix


def reduce_column(matrix: list[list[int]], col: int) -> None:
    assert col > 0
    new_entries = [
        min(
            sum(
                matrix[cell][col - 1]
                for cell in range(min(row, target), max(row, target) + 1)
            )
            + matrix[target][col]
            for target in range(len(matrix))
        )
        for row in range(len(matrix))
    ]
    for row, value in enumerate(new_entries):
        matrix[row][col - 1] = value


def three_way_path_sum(matrix: list[list[int]]) -> int:
    for col in range(len(matrix) - 1, 0, -1):
        reduce_column(matrix, col)
    return min(matrix[row][0] for row in range(len(matrix)))


def solution() -> None:
    matrix = read_matrix("data/p082_matrix.txt")
    return three_way_path_sum(matrix)


if __name__ == "__main__":
    import runner

    runner.run(globals())
