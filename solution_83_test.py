from solution_83 import four_way_path_sum


def test_four_way_path_sum() -> None:
    matrix = [
        [131, 673, 234, 103, 18],
        [201, 96, 342, 965, 150],
        [630, 803, 746, 422, 111],
        [537, 599, 497, 121, 956],
        [805, 732, 524, 37, 331],
    ]
    assert four_way_path_sum(matrix) == 2297
