package gigasecond

import "math"
import "time"

func AddGigasecond(t time.Time) time.Time {
	gigasecond := time.Duration(math.Pow(10, 9)) * time.Second
	return t.Add(gigasecond)
}
