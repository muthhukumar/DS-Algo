/**
 * My first solution without looking any solutions
 * @param {character[][]} board
 * @return {boolean}
 */
var isValidSudoku = function (board) {
  const row = new Map()
  const col = new Map()
  const box = new Map()

  for (let i = 0; i < 9; i++) {
    for (let j = 0; j < 9; j++) {
      const curr = board[i][j]

      if (curr === ".") continue

      const idx = String(Math.floor(i / 3)) + String(Math.floor(j / 3))

      if (
        (row.has(i) && row.get(i).has(curr)) ||
        (col.has(j) && col.get(j).has(curr)) ||
        (box.has(idx) && box.get(idx).has(curr))
      ) {
        return false
      } else {
        if (!box.has(idx)) box.set(idx, new Map())
        box.get(idx).set(curr, idx)

        if (!row.has(i)) row.set(i, new Map())
        row.get(i).set(curr, i)

        if (!col.has(j)) col.set(j, new Map())
        col.get(j).set(curr, j)
      }
    }
  }

  return true
}

/**
 * My most optimal solution. This solution is inspired from multiple solutions from others. Like prefilling the object to eliminate if check and using set so that we don't have to set any value
 * @param {character[][]} board
 * @return {boolean}
 */
var isValidSudoku = function (board) {
  const row = {}
  const col = {}
  const box = {}

  for (let i = 0; i < 9; i++) {
    row[i] = new Set()
    col[i] = new Set()
    box[i] = new Set()
  }

  for (let i = 0; i < 9; i++) {
    for (let j = 0; j < 9; j++) {
      const curr = board[i][j]

      if (curr === ".") continue

      // Considering each 3x3 as single index from 0 - 9
      const idx = Math.floor(i / 3) * 3 + Math.floor(j / 3)

      if (row[i].has(curr) || col[j].has(curr) || box[idx].has(curr)) {
        return false
      } else {
        box[idx].add(curr)
        row[i].add(curr)
        col[j].add(curr)
      }
    }
  }

  return true
}
