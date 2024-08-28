function evalRPN(tokens: string[]): number {
  const stack: Array<number> = [];

  // It is guarantee to have one number
  stack.push(Number(tokens[0]));

  for (let i = 1; i < tokens.length; i++) {
    switch (tokens[i]) {
      case "+":
        {
          const a = stack.pop();
          const b = stack.pop();

          stack.push(a + b);
        }
        break;
      case "-":
        {
          const a = stack.pop();
          const b = stack.pop();

          stack.push(b - a);
        }
        break;
      case "*":
        {
          const a = stack.pop();
          const b = stack.pop();

          stack.push(a * b);
        }
        break;
      case "/":
        {
          const a = stack.pop();
          const b = stack.pop();

          // Trunc removes fractional value and returns integer value of that number
          stack.push(Math.trunc(b / a));
        }
        break;
      default:
        stack.push(Number(tokens[i]));
    }
  }

  return stack.pop();
}
