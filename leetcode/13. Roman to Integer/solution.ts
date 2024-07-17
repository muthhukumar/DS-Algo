function romanToInt(s: string): number {
  let idx = 0

  let result = 0

  while (idx < s.length) {
    switch (s[idx]) {
      case "I": {
        if (idx + 1 <= s.length && s[idx + 1] === "V") {
          result += 4
          idx += 1
        } else if (idx + 1 <= s.length && s[idx + 1] === "X") {
          result += 9
          idx += 1
        } else {
          result += 1
        }

        break
      }
      case "V":
        result += 5
        break
      case "X": {
        if (idx + 1 <= s.length && s[idx + 1] === "L") {
          result += 40
          idx += 1
        } else if (idx + 1 <= s.length && s[idx + 1] === "C") {
          result += 90
          idx += 1
        } else {
          result += 10
        }

        break
      }
      case "L":
        result += 50
        break
      case "C": {
        if (idx + 1 <= s.length && s[idx + 1] === "D") {
          result += 400
          idx += 1
        } else if (idx + 1 <= s.length && s[idx + 1] === "M") {
          result += 900
          idx += 1
        } else {
          result += 100
        }
        break
      }
      case "D":
        result += 500
        break
      case "M":
        result += 1000
        break
    }

    idx += 1
  }

  return result
}
