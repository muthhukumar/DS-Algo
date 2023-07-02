const DIR: [[i32; 2]; 4] = [[-1, 0], [1, 0], [0, -1], [0, 1]];

#[derive(Debug)]
pub struct Point {
    x: usize,
    y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Point {
        Point { x, y }
    }

    pub fn val_n_get_pt(&self, dir: &[i32; 2]) -> Option<Point> {
        let val_point = match dir {
            [x, y] if *x == -1 && *y == 0 => (self.x.checked_sub(1), self.y.checked_add(0)),
            [x, y] if *x == 1 && *y == 0 => (self.x.checked_add(1), self.y.checked_add(0)),
            [x, y] if *x == 0 && *y == -1 => (self.x.checked_add(0), self.y.checked_sub(1)),
            [x, y] if *x == 0 && *y == 1 => (self.x.checked_add(0), self.y.checked_add(1)),
            [_, _] => (None, None),
        };

        match val_point {
            (Some(x), Some(y)) => Some(Point::new(x, y)),
            _ => None,
        }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }

    fn ne(&self, other: &Self) -> bool {
        self.x != other.x || self.y != other.y
    }
}

impl Clone for Point {
    fn clone(&self) -> Self {
        Point {
            x: self.x,
            y: self.y,
        }
    }
}

fn walk(
    maze: &Vec<Vec<char>>,
    wall: char,
    curr: &Point,
    end: &Point,
    seen: &mut Vec<Vec<bool>>,
    path: &mut Vec<Point>,
) -> bool {
    // Of the map
    if curr.x >= maze[0].len() || curr.y >= maze.len() {
        return false;
    }

    // Reached
    if curr == end {
        path.push(curr.clone());
        return true;
    }

    // wall
    if maze[curr.y][curr.x] == wall {
        return false;
    }

    // already visited
    if seen[curr.y][curr.x] {
        return false;
    }

    path.push(curr.clone());
    seen[curr.y][curr.x] = true;

    for d in DIR {
        if let Some(point) = curr.val_n_get_pt(&d) {
            if walk(maze, wall, &point, end, seen, path) {
                return true;
            }
        }
    }

    path.pop();

    false
}

pub fn solve(maze: Vec<Vec<char>>, wall: char, start: Point, end: Point) -> Vec<Point> {
    let mut seen: Vec<Vec<bool>> = vec![vec![false; maze[0].len()]; maze.len()];

    let mut paths: Vec<Point> = vec![];

    walk(&maze, wall, &start, &end, &mut seen, &mut paths);

    paths
}

#[cfg(test)]
mod tests {

    use super::{solve, Point};

    #[test]
    fn solve_maze() {
        let maze_str: Vec<&str> = vec![
            "xxxxxxxxxx x",
            "x        x x",
            "x        x x",
            "x xxxxxxxx x",
            "x          x",
            "x xxxxxxxxxx",
        ];

        let maze: Vec<Vec<char>> = maze_str.iter().map(|s| s.chars().collect()).collect();

        let maze_result: Vec<Point> = vec![
            Point::new(10, 0),
            Point::new(10, 1),
            Point::new(10, 2),
            Point::new(10, 3),
            Point::new(10, 4),
            Point::new(9, 4),
            Point::new(8, 4),
            Point::new(7, 4),
            Point::new(6, 4),
            Point::new(5, 4),
            Point::new(4, 4),
            Point::new(3, 4),
            Point::new(2, 4),
            Point::new(1, 4),
            Point::new(1, 5),
        ];

        let actual_result = solve(maze, 'x', Point::new(10, 0), Point::new(1, 5));

        assert_eq!(maze_result, actual_result);
    }
}
