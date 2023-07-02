function qs(arr: number[], lo: number, hi: number): void {
  if (lo >= hi) {
    return;
  }

  let pivotIdx = partition(arr, lo, hi);

  qs(arr, lo, pivotIdx - 1);
  qs(arr, pivotIdx + 1, hi);
}

function partition(arr: number[], lo: number, hi: number): number {
  const pivot = arr[hi];

  let idx = lo - 1;

  for (let i = lo; i < hi; ++i) {
    if (arr[i] <= pivot) {
      idx++;

      const temp = arr[i];
      arr[i] = arr[idx];
      arr[idx] = temp;
    }
  }

  idx++;

  arr[hi] = arr[idx];
  arr[idx] = pivot;

  return idx;
}

const input = [9, 1, 4, 8, 3, 6];

// input === [1, 3, 4, 6, 8, 9];
