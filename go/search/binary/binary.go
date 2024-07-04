package binary

import "math"

func BinarySearch(haystack []int64, needle int64) bool {
	var low, high int64 = 0, int64(len(haystack))

	for low < high {
		mid := int64(math.Floor(float64(low + (high-low)/2)))

		val := haystack[mid]

		if val == needle {
			return true
		} else if needle > val {
			low = mid + 1
		} else {
			high = mid
		}

	}

	return false
}
