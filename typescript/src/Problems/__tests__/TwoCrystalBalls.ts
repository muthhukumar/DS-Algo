import { twoCrystalBalls } from "~/Problems";

test("two crystal balls", function () {
  let idx = Math.floor(Math.random() * 10000);
  const data = new Array(10000).fill(false);

  for (let i = idx; i < 10000; ++i) {
    data[i] = true;
  }

  expect(twoCrystalBalls(data)).toEqual(idx);
  expect(twoCrystalBalls(new Array(821).fill(false))).toEqual(-1);
});
