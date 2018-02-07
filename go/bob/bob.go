package bob

import "strings"
import "regexp"

func Hey(remark string) string {
	remark_ := strings.TrimSpace(remark)
	is_question := IsQuestion(remark_)
	is_upper := IsUpper(remark_)

	if IsBlank(remark_) {
		return "Fine. Be that way!"
	} else if is_question && is_upper {
		return "Calm down, I know what I'm doing!"
	} else if is_question {
		return "Sure."
	} else if is_upper {
		return "Whoa, chill out!"
	} else {
		return "Whatever."
	}
}

func IsBlank(str string) bool {
	return str == ""
}

func IsUpper(str string) bool {
	is_lower := strings.ToLower(str) == str
	return !is_lower && strings.ToUpper(str) == str
}

func IsQuestion(str string) bool {
	question_regex := regexp.MustCompile("\\?$")
	return question_regex.MatchString(str)
}
