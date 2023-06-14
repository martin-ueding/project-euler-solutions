from runner import format_timing


def test_format_timing() -> None:
    assert format_timing(121) == "2:01 min"
    assert format_timing(0.1234) == "123 ms"
    assert format_timing(0.0001234) == "123 µs"
    assert format_timing(0.001234) == "1.2 ms"
