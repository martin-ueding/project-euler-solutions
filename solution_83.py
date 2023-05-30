from solution_81 import read_matrix


def four_way_path_sum(matrix: list[list[int]]) -> int:
    size = len(matrix)
    node_weights = {
        (row, col): matrix[row][col] for row in range(size) for col in range(size)
    }
    infinity = sum(node_weights.values()) + 1
    unvisited = {(row, col) for row in range(size) for col in range(size)}
    distances = {(row, col): infinity for row in range(size) for col in range(size)}
    distances[(0, 0)] = matrix[0][0]
    target = (size - 1, size - 1)
    while target in unvisited:
        current = min(unvisited, key=lambda node: distances[node])
        current_row, current_col = current
        up = (current_row - 1, current_col)
        down = (current_row + 1, current_col)
        left = (current_row, current_col - 1)
        right = (current_row, current_col + 1)

        for neighbor in [up, down, left, right]:
            if neighbor in unvisited:
                distances[neighbor] = min(
                    distances[neighbor], distances[current] + node_weights[neighbor]
                )

        unvisited.remove(current)

    return distances[target]


def solution() -> None:
    matrix = read_matrix("data/p083_matrix.txt")
    return four_way_path_sum(matrix)


if __name__ == "__main__":
    import runner

    runner.run(globals())
