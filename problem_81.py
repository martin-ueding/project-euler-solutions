import json
from typing import Iterator


def read_matrix(filename: str) -> list[list[int]]:
    result = []
    with open(filename) as f:
        for line in f:
            result.append(json.loads(f"[{line}]"))
    return result


def iter_diagonally(size: int) -> Iterator[tuple[int, int]]:
    row = size - 1
    col = size - 1
    while True:
        yield row, col
        col += 1
        row -= 1
        if row < 0:
            row = col - 2
            col = 0
            if row < 0:
                return
        if col >= size:
            col = row
            row = size - 1


def solution() -> None:
    matrix = read_matrix("data/p081_matrix.txt")
    size = 80
    for row, col in iter_diagonally(size):
        neighbors = []
        if row < size - 1:
            neighbors.append(matrix[row + 1][col])
        if col < size - 1:
            neighbors.append(matrix[row][col + 1])
        matrix[row][col] += min(neighbors, default=0)
    return matrix[0][0]


if __name__ == "__main__":
    import runner

    runner.run(globals())
