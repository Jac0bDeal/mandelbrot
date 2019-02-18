package main

import (
	"fmt"
	"log"
	"math/cmplx"
	"os"
)

func main() {
	const filename = "mandelbrot.ppm"
	const width, height float64 = 2000, 2000
	const radius float64 = 2

	f, err := os.Create(filename)
	if err != nil {
		log.Fatalf("could not open %s due to: %v", filename, err)
	}
	defer f.Close()

	_, err = f.WriteString(fmt.Sprintf("P3\n%v %v 255\n", width, height))
	if err != nil {
		log.Fatalf("failed to write file header due to: %v", err)
	}

	for y := float64(0); y < height; y++ {
		for x := float64(0); x < width; x++ {
			c := complex((2*x/width-1)*radius, (2*y/height-1)*radius)

			z := complex(0, 0)
			var iter uint
			for iter = 0; cmplx.Abs(z) < 2 && iter <= 34; iter++ {
				z = z*z + c
			}
			var val uint
			if iter < 34 {
				val = (255 * iter) / 33
			} else {
				val = 0
			}

			_, err := f.WriteString(fmt.Sprintf("%v %v %v\n", val, 0, 0))
			if err != nil {
				log.Fatalf("failed to write line due to: %v", err)
			}
		}
	}
}
