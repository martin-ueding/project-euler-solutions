from problem_76 import num_partitions, partitions


def test_partitions() -> None:
    assert num_partitions(2) == 2
    assert num_partitions(3) == 3
    assert num_partitions(4) == 5
    assert num_partitions(5) == 7
