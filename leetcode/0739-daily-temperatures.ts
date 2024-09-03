// Optimal solution by using monotonic stack
function dailyTemperatures(temperatures: number[]): number[] {
  const result: Array<number> = new Array(temperatures.length).fill(0);

  const stack = [];

  let idx = 0;

  while (idx < temperatures.length) {
    const temp = temperatures[idx];

    if (stack.length > 0 && temperatures[stack[stack.length - 1]] < temp) {
      const last = stack.pop();

      result[last] = idx - last;

      continue;
    } else {
      stack.push(idx);
    }

    idx++;
  }

  return result;
}

// Brute force approach
function dailyTemperatures2(temperatures: number[]): number[] {
  const result: Array<number> = [];

  for (let i = 0; i < temperatures.length; i++) {
    let breaked = false;

    let count = 0;
    for (let j = i + 1; j < temperatures.length; j++) {
      count += 1;
      if (temperatures[j] > temperatures[i]) {
        result[i] = count;
        breaked = true;
        break;
      }
    }

    if (!breaked) {
      result[i] = 0;
    }
  }

  return result;
}
