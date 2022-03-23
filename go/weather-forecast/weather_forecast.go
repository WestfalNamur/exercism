// Package weather Package that describes the current weather.
package weather

// CurrentCondition Stores current weather condition.
var CurrentCondition string

// CurrentLocation Stores current location.
var CurrentLocation string

// Forecast Calculate weather forecast.
func Forecast(city, condition string) string {
	CurrentLocation, CurrentCondition = city, condition
	return CurrentLocation + " - current weather condition: " + CurrentCondition
}
