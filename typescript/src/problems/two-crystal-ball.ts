function two_crystal_balls(arr: Array<boolean>): number {
  let jump = Math.sqrt(arr.length);

  let i = 0;

  for (; i < arr.length; i += jump) {
    if (arr[i]) {
      break;
    }
  }

  let start = i - jump;

  for (let j = start; j < i + jump && j < arr.length; j++) {
    if (arr[j]) {
      return j;
    }
  }

  return -1;
}
