package linear

func LinearSearch(haystack []int64, needle int64) bool {
	for _, el := range haystack {
		if el == needle {
			return true
		}
	}

	return false
}
