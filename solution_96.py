from collections.abc import Iterator


def iter_problems() -> Iterator[list[list[int]]]:
    with open('data/p096_sudoku.txt') as f:
        rows = []
        for line in f:
            if line.startswith('Grid'):
                rows = []
            else:
                rows.append([int(d) for d in line.strip()])
            if len(rows) == 9:
                yield rows


def solution() -> int:
    for problem in iter_problems():
        print(problem)
        return 0



if __name__ == "__main__":
    import runner

    runner.run(globals())
