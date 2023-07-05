import { inOrderSearch } from "..";
import { tree } from "../tree";

test("In order", function () {
  expect(inOrderSearch(tree)).toEqual([5, 7, 10, 15, 20, 29, 30, 45, 50, 100]);
});
