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
    print('┌───────┬───────┬───────┐')
    for rb in range(3):
        if rb > 0:
            print('├───────┼───────┼───────┤')
        for r in range(3):
            for cb in range(3):
                print('│', end='')
                for c in range(3):
                    print(' ', end='')
                    print(grid[rb *3 + r][cb * 3 + c] or ' ', end='')
                print(' ', end='')
            print('│')
    print('└───────┴───────┴───────┘')

def solution() -> int:
    for grid in iter_problems():
        print(grid)
        print_grid(grid)

        print(
            sorted(
                collections.Counter(
                    len(get_possibilities(grid, row, col))
                    for row in range(9)
                    for col in range(9)
                ).items()
            )
        )

        return 0


if __name__ == "__main__":
    import runner

    solution()

    # runner.run(globals())
