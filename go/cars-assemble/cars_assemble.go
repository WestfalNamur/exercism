package cars

// CalculateWorkingCarsPerHour calculates how many working cars are
// produced by the assembly line every hour.
//
// Implement a function that takes in the number of cars produced per hour and the success rate and calculates the
// number of successful cars made per hour. The success rate is given as a percentage, from 0 to 100.
func CalculateWorkingCarsPerHour(productionRate int, successRate float64) float64 {
	return successRate * float64(productionRate) / 100
}

// CalculateWorkingCarsPerMinute calculates how many working cars are
// produced by the assembly line every minute
func CalculateWorkingCarsPerMinute(productionRate int, successRate float64) int {
	return int(float64(productionRate/60)*successRate) / 100
}

// CalculateCost works out the cost of producing the given number of cars
//
// Each car normally costs $10,000 to produce individually, regardless of whether it is successful or not. But with a
// bit of planning, 10 cars can be produced together for $95,000.
// For example, 37 cars can be produced in the following way: 37 = 3 x groups of ten + 7 individual cars
// So the cost for 37 cars is: 3*95,000+7*10,000=355,000
//
// Implement the function CalculateCost that calculates the cost of producing a number of cars, regardless of whether
// they are successful:
func CalculateCost(carsCount int) uint {
	modulo := carsCount % 10
	rest := carsCount - modulo
	return uint(modulo*10000 + rest*9500)
}
