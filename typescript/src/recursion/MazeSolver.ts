const dir = [
  [-1, 0],
  [1, 0],
  [0, -1],
  [0, 1],
];

function walk(
  maze: string[],
  wall: string,
  curr: Point,
  end: Point,
  seen: boolean[][],
  paths: Point[]
): boolean {
  // Off the map
  if (
    curr.x < 0 ||
    curr.x >= maze[0].length ||
    curr.y < 0 ||
    curr.y >= maze.length
  ) {
    return false;
  }

  // on wall
  if (maze[curr.y][curr.x] === wall) {
    return false;
  }

  // end
  if (curr.x === end.x && curr.y === end.y) {
    paths.push(curr);

    return true;
  }

  // already visited
  if (seen[curr.y][curr.x]) {
    return false;
  }

  // pre recurse
  paths.push(curr);
  seen[curr.y][curr.x] = true;

  for (let i = 0; i < dir.length; ++i) {
    let [x, y] = dir[i];

    let start = {
      x: curr.x + x,
      y: curr.y + y,
    };

    // recursion
    if (walk(maze, wall, start, end, seen, paths)) {
      return true;
    }
  }

  // post recursion
  paths.pop();

  return false;
}

export default function solve(
  maze: string[],
  wall: string,
  start: Point,
  end: Point
): Point[] {
  const seen: boolean[][] = Array.from({ length: maze.length }).map((_, i) =>
    Array.from({ length: maze[i].length }).fill(false)
  ) as boolean[][];

  const paths: Point[] = [];

  walk(maze, wall, start, end, seen, paths);

  return paths;
}
