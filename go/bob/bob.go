package bob

import "strings"
import "regexp"

func Hey(remark string) string {
	remark_ := strings.TrimSpace(remark)
	question := isQuestion(remark_)
	upper := isUpper(remark_)

	if isBlank(remark_) {
		return "Fine. Be that way!"
	} else if question && upper {
		return "Calm down, I know what I'm doing!"
	} else if question {
		return "Sure."
	} else if upper {
		return "Whoa, chill out!"
	} else {
		return "Whatever."
	}
}

func isBlank(str string) bool {
	return str == ""
}

func isUpper(str string) bool {
	is_lower := strings.ToLower(str) == str
	return !is_lower && strings.ToUpper(str) == str
}

func isQuestion(str string) bool {
	question_regex := regexp.MustCompile("\\?$")
	return question_regex.MatchString(str)
}
