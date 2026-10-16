package raindrops

import (
	"strconv"
	"strings"
)

func Convert(number int) string {
	rules := []struct {
		divisor int
		word    string
	}{
		{3, "Pling"},
		{5, "Plang"},
		{7, "Plong"},
	}

	var sb strings.Builder
	for _, rule := range rules {
		if number%rule.divisor == 0 {
			sb.WriteString(rule.word)
		}
	}

	if sb.Len() == 0 {
		return strconv.Itoa(number)
	}
	return sb.String()
}
