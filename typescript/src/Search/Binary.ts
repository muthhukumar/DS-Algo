/*
 * Binary search assumes that the input array is sorted
 */
export default function binarySearch(sortedArr: Array<number>, needle: number) {
  let low = 0;
  let high = sortedArr.length;

  while (low < high) {
    const middle = Math.floor(low + (high - low) / 2);

    let value = sortedArr[middle];

    if (value === needle) {
      return true;
    } else if (needle > value) {
      low = middle + 1;
    } else {
      high = middle;
    }
  }

  return false;
}
