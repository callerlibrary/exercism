// Package weather provides simple weather information.
package weather

var (
	// CurrentCondition holds the latest weather condition.
	CurrentCondition string

	// CurrentLocation holds the latest city name.
	CurrentLocation string
)

// Forecast returns a formatted weather description.
func Forecast(city, condition string) string {
	CurrentLocation, CurrentCondition = city, condition
	return CurrentLocation + " - current weather condition: " + CurrentCondition
}
