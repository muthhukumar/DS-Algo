let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

/*
 * Binary search assumes that the input array is sorted
 */
function binarySearch(sortedArr: Array<number>, needle: number) {
  let low = 0;
  let high = sortedArr.length;

  while (low < high) {
    const middle = Math.floor(low + (high - low) / 2);

    if (sortedArr[middle] == needle) {
      return true;
    } else if (needle > middle) {
      low = middle + 1;
    } else {
      high = middle;
    }
  }

  return false;
}
