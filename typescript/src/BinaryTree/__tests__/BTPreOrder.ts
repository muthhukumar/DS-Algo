import { preOrderSearch } from "..";
import { tree } from "../tree";

test("Pre order", function () {
  expect(preOrderSearch(tree)).toEqual([20, 10, 5, 7, 15, 50, 30, 29, 45, 100]);
});
