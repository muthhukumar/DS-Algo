function sortTheStudents(score: number[][], k: number): number[][] {
  for (let i = 0; i < score.length; i++) {
    for (let j = i + 1; j < score.length; j++) {
      if (score[i][k] < score[j][k]) {
        const temp = score[i]
        score[i] = score[j]
        score[j] = temp
      }
    }
  }

  return score
}
