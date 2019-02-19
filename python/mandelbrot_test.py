#!/usr/bin/env python3
from .mandelbrot import get_color_value
import unittest


class TestGetColorValue(unittest.TestCase):
    def test_inside_set(self):
        val = get_color_value(-1 + 0j)
        self.assertEqual(val, 0)

    def test_outside_test(self):
        val = get_color_value(4 + 4j)
        self.assertEqual(val, 7)

    def test_escapes_set(self):
        val = get_color_value(1 + 1j)
        self.assertEqual(val, 15)


if __name__ == '__main__':
    unittest.main()
