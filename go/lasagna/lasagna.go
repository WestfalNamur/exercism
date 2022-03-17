package lasagna

// TODO: define the 'OvenTime' constant
//
// You have four tasks, all related to the time spent cooking the lasagna.
// 1. Define the expected oven time in minutes (DONE)
// 2. Calculate the remaining oven time in minutes (DONE)
// 3. Calculate the preparation time in minutes
// 4. Calculate the elapsed working time in minutes

const OvenTime = 40

// RemainingOvenTime returns the remaining minutes based on the `actual` minutes already in the oven.
func RemainingOvenTime(actualMinutesInOven int) int {
	return OvenTime - actualMinutesInOven
}

// PreparationTime calculates the time needed to prepare the lasagna based on the amount of layers.
// assuming each layer takes you 2 minutes to prepare.
func PreparationTime(numberOfLayers int) int {
	return numberOfLayers * 2
}

// ElapsedTime calculates the total time needed to create and bake a lasagna.
// The function should return how many minutes in total you've worked on cooking the lasagna, which is the sum of
// the preparation time in minutes, and the time in minutes the lasagna has spent in the oven at the moment.
func ElapsedTime(numberOfLayers, actualMinutesInOven int) int {
	preparationTime := PreparationTime(numberOfLayers)
	return preparationTime + actualMinutesInOven
}
