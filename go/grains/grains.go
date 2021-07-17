package grains

import "fmt"

func Square(n int) (uint64, error) {
	if n < 1 || n > 64 {
		return 0, fmt.Errorf("must be between 1 and 64")
	}
	// Binary shift
	// Simple: n << x is "n times 2, x times". And y >> z is "y divided by 2, z times".
	// We can use binary shift here as the grains on the board grow to the power
	// of 2 (Binary has the base of 2).
	// 0010001 << 2 == 1000100 in decimal 17;
	return 1 << (n - 1), nil
}

func Total() uint64 {
	return 1<<64 - 1
}
