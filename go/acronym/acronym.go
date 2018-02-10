package acronym

import "strings"

func Abbreviate(phrase string) string {
	dashlessPhrase := strings.Replace(phrase, "-", " ", -1)
	words := strings.Split(dashlessPhrase, " ")

	firstLetters := make([]string, 20)
	for index, word := range words {
		firstChar := string(word[0])
		firstLetters[index] = strings.ToUpper(firstChar)
	}

	return strings.Join(firstLetters, "")
}
