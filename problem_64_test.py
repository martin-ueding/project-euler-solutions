from problem_64 import expand_root


def test_expansion() -> None:
    expand_root(2) == ([1], [2])
    expand_root(3) == ([1], [1, 2])
    expand_root(5) == ([2], [4])
    expand_root(6) == ([2], [2, 4])
    expand_root(7) == ([2], [1, 1, 1, 4])
    expand_root(8) == ([2], [1, 4])
    expand_root(10) == ([3], [6])
    expand_root(11) == ([3], [3, 6])
    expand_root(12) == ([3], [2, 6])
    expand_root(13) == ([3], [1, 1, 1, 1, 6])
    expand_root(23) == ([4], [1, 3, 1, 8])
