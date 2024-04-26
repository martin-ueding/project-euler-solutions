import collections
from collections.abc import Iterator


Grid = list[list[int]]


def iter_problems() -> Iterator[list[list[int]]]:
    with open("data/p096_sudoku.txt") as f:
        rows = []
        for line in f:
            if line.startswith("Grid"):
                rows = []
            else:
                rows.append([int(d) for d in line.strip()])
            if len(rows) == 9:
                yield rows


def iter_square(grid: Grid, row: int, col: int) -> Iterator[int]:
    row -= row % 3
    col -= col % 3
    for i in range(3):
        for j in range(3):
            yield grid[row + i][col + j]


def get_possibilities(grid: Grid, row: int, col: int) -> set[int]:
    if grid[row][col]:
        return set()

    possible = set(range(1, 10))

    for c in range(9):
        possible.discard(grid[row][c])
    for r in range(9):
        possible.discard(grid[r][col])
    for elem in iter_square(grid, row, col):
        possible.discard(elem)

    return possible


def print_grid(grid) -> None:
    print("┌───────┬───────┬───────┐")
    for rb in range(3):
        if rb > 0:
            print("├───────┼───────┼───────┤")
        for r in range(3):
            for cb in range(3):
                print("│", end="")
                for c in range(3):
                    print(" ", end="")
                    print(grid[rb * 3 + r][cb * 3 + c] or " ", end="")
                print(" ", end="")
            print("│")
    print("└───────┴───────┴───────┘")


def fill_in(grid: Grid, row: int, col: int):
    if row >= 9:
        return True

    if grid[row][col]:
        return fill_in(grid, row + 1 if col == 8 else row, (col + 1) % 9)

    possibilities = get_possibilities(grid, row, col)
    for possibility in possibilities:
        grid[row][col] = possibility
        if fill_in(grid, row + 1 if col == 8 else row, (col + 1) % 9):
            return True
        grid[row][col] = 0
    return False


def grid_number(grid) -> int:
    for row in range(9):
        for col in range(9):
            assert grid[row][col], grid
    return int("".join(map(str, grid[0][0:3])))


def solution() -> int:
    s = 0
    for grid in iter_problems():
        assert fill_in(grid, 0, 0)
        s += grid_number(grid)
    return s


if __name__ == "__main__":
    import runner

    runner.run(globals())
