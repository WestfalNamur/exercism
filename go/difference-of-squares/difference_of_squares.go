package diffsquares

// Find the difference between the square of the sum and the sum of the squares of the first N natural numbers.
// The square of the sum of the first ten natural numbers is (1 + 2 + ... + 10)² = 55² = 3025.
// The sum of the squares of the first ten natural numbers is 1² + 2² + ... + 10² = 385

func makeRange(min, max int) []int {
	a := make([]int, max-min+1)
	for i := range a {
		a[i] = min + i
	}
	return a
}

func SquareOfSum(n int) int {
	a := makeRange(0, n)
	sum := 0
	for _, v := range a {
		sum += v
	}
	return sum * sum
}

func SumOfSquares(n int) int {
	a := makeRange(0, n)
	sum := 0
	for _, v := range a {
		sum += v * v
	}
	return sum
}

func Difference(n int) int {
	return SquareOfSum(n) - SumOfSquares(n)
}
