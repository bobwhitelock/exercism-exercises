package twofer

func ShareWith(person string) string {
	if person == "" {
		person = "you"
	}
	return "One for " + person + ", one for me."
}
