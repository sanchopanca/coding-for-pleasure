use aoc2023::*;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Pipe(Direction, Direction);

impl Pipe {
    fn from_char(c: char) -> Self {
        match c {
            '|' => Pipe(Direction::Down, Direction::Up),
            '-' => Pipe(Direction::Left, Direction::Right),
            'L' => Pipe(Direction::Right, Direction::Up),
            'J' => Pipe(Direction::Left, Direction::Up),
            '7' => Pipe(Direction::Down, Direction::Left),
            'F' => Pipe(Direction::Down, Direction::Right),
            _ => panic!(),
        }
    }
}

fn main() {
    let maze = read_input_to_char_vectors(10);
    let mut start = (0, 0);
    for (y, row) in maze.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == 'S' {
                start = (y, x);
            }
        }
    }

    for pipe in ['|', '-', 'L', 'J', '7', 'F'] {
        let mut maze_copy = maze.clone();
        maze_copy[start.0][start.1] = pipe;
        if let Some((l, visited)) = loop_length(&maze_copy, start) {
            println!("{}", l / 2);
            println!("{}", find_inside_area(maze_copy, visited));
        }
    }
}

fn loop_length(maze: &Vec<Vec<char>>, start: (usize, usize)) -> Option<(u32, Vec<Vec<bool>>)> {
    let mut steps = 0;
    let mut visited = vec![vec![false; maze[0].len()]; maze.len()];

    let mut current = start;
    let mut came_from = Direction::Left;
    while !visited[current.0][current.1] {
        visited[current.0][current.1] = true;
        if maze[current.0][current.1] == '.' {
            return None;
        }
        let pipe = Pipe::from_char(maze[current.0][current.1]);
        let next = if pipe.0 != came_from { pipe.0 } else { pipe.1 };

        match next {
            Direction::Up => {
                if current.0 == 0 {
                    return None;
                }
                current.0 -= 1;
            }
            Direction::Down => {
                if current.0 == maze.len() - 1 {
                    return None;
                }
                current.0 += 1;
            }
            Direction::Left => {
                if current.1 == 0 {
                    return None;
                }
                current.1 -= 1;
            }
            Direction::Right => {
                if current.1 == maze[0].len() - 1 {
                    return None;
                }
                current.1 += 1;
            }
        }

        came_from = next.opposite();
        steps += 1;
    }
    let start_pipe = Pipe::from_char(maze[start.0][start.1]);
    if start_pipe.0 == came_from || start_pipe.1 == came_from {
        return Some((steps, visited));
    }
    None
}

fn find_inside_area(maze: Vec<Vec<char>>, visited: Vec<Vec<bool>>) -> u32 {
    let mut area = 0;
    for (i, row) in visited.iter().enumerate() {
        let mut inside = false;
        for (j, &cell) in row.iter().enumerate() {
            let m = maze[i][j];
            if cell && (m == '|' || m == 'L' || m == 'J') {
                inside = !inside;
                continue;
            }
            if inside && !cell {
                area += 1;
            }
        }
    }
    area
}
