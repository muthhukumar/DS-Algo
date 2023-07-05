import { compareTwoBinaryTrees } from "..";
import { tree, tree2 } from "../tree";

test("Compare Binary Trees", function () {
  expect(compareTwoBinaryTrees(tree, tree)).toEqual(true);
  expect(compareTwoBinaryTrees(tree, tree2)).toEqual(false);
});
