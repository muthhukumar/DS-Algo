function fibonacci(n, mem) {
  if (mem[n] >= 0) {
    return mem[n];
  }

  if (n <= 1) {
    return n;
  }

  mem[n - 1] = fibonacci(n - 2, mem) + fibonacci(n - 1, mem);

  return mem[n - 1];
}

function fibonacci2(n) {
  if (n <= 1) {
    return n;
  }

  return fibonacci2(n - 2) + fibonacci2(n - 1);
}

const n = 5000;
const mem = new Array(n + 1).fill(-1);

const [first, second] = [true, false];

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
