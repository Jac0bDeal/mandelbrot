package main

import "testing"

func Test_getColorValueInsideSet(t *testing.T) {
	// test that -1 + 0i -> 255
	val := getColorValue(complex(-1, 0))
	if val != 0 {
		t.Errorf("value for -1 + 1i is incorrect, got: %d, want: %d", val, 0)
	}
}

func Test_getColorValueOutsideSet(t *testing.T) {
	// test that 4 + 4i -> 7
	val := getColorValue(complex(4, 4))
	if val != 7 {
		t.Errorf("value for 4 + 4i is incorrect, got: %d, want %d", val, 7)
	}
}

func Test_getColorValueEscapesSet(t *testing.T) {
	// test that 1 + 1i -> 15
	val := getColorValue(complex(1, 1))
	if val != 15 {
		t.Errorf("value for 1 + 1i is incorrect, got: %d, want %d", val, 15)
	}
}
