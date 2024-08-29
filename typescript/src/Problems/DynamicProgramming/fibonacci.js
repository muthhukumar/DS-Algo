// Using memoization
function fibonacci(n, mem) {
  if (mem[n] >= 0) {
    return mem[n];
  }

  if (n <= 1) {
    return n;
  }

  mem[n] = fibonacci(n - 2, mem) + fibonacci(n - 1, mem);

  return mem[n];
}

// Normal recursion
function fibonacci2(n) {
  if (n <= 1) {
    return n;
  }

  return fibonacci2(n - 2) + fibonacci2(n - 1);
}

// Using tabulation
function fibonacci3(n) {
  if (n <= 1) {
    return n;
  }

  const mem = new Array(n + 1).fill(-1);

  mem[0] = 0;
  mem[1] = 1;

  for (let i = 2; i <= n; i++) {
    mem[i] = mem[i - 2] + mem[i - 1];
  }

  return mem[n];
}

const n = 5;
const mem = new Array(n + 1).fill(-1);

const [first, second, third] = [true, true, true];

if (first) {
  console.time("Fibonacci Time");
  console.log(fibonacci(n, mem));
  console.timeEnd("Fibonacci Time");
}

if (second) {
  console.time("Fibonacci Time");
  console.log(fibonacci2(n));
  console.timeEnd("Fibonacci Time");
}

if (third) {
  console.time("Fibonacci Time");
  console.log(fibonacci3(n));
  console.timeEnd("Fibonacci Time");
}
