package binary

import "testing"

func TestFound(t *testing.T) {
	found := BinarySearch([]int64{32, 5, 1, 2, 5, 667, 7, 23, 5}, 23)

	if found != true {
		t.Fatal("22 should be there")
	}
}

func TestNotFound(t *testing.T) {
	found := BinarySearch([]int64{32, 5, 1, 2, 5, 667, 7, 23, 5}, 22)

	if found != false {
		t.Fatal("22 should not be there")
	}
}
