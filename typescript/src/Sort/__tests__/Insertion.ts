import { insertionSort } from "~/Sort";

test("insertion-sort", function () {
  const arr = [9, 3, 7, 4, 69, 420, 42];

  insertionSort(arr);
  expect(arr).toEqual([3, 4, 7, 9, 42, 69, 420]);
});
