package hamming

import (
	"errors"
)

func Distance(a, b string) (int, error) {
	if len(a) != len(b) {
		return -1, errors.New("a and b not of equal length")
	}

	n := 0

	for i := range a {
		if a[i] != b[i] {
			n++
		}
	}

	return n, nil
}
