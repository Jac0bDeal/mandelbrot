#!/usr/bin/env python3
import math


def main():
    filename = "mandelbrot.ppm"
    width, height = 1200, 1200
    radius = 2.0

    with open(filename, 'w') as f:
        f.write(f"P3\n{width} {height} 255\n")

        for y in range(height):
            for x in range(height):
                real = (2 * x/width - 1) * radius
                imag = (2 * y/height - 1) * radius
                point = complex(real, imag)
                val = get_color_value(point)

                f.write(f"{val} 0 0\n")


def get_color_value(c: complex) -> int:
    z = 0 + 0j

    iteration = 0
    while abs(z) < 2 and iteration <= 34:
        z = z*z + c
        iteration += 1

    return math.floor((255 * iteration) / 33) if iteration < 34 else 0


if __name__ == "__main__":
    main()
