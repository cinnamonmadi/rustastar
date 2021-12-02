use std::fmt;

#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
struct Point {
    x: i32,
    y: i32
}

const DIRECTIONS: [Point; 4] = [
    Point { x: 0, y: -1 },
    Point { x: 1, y: 0 },
    Point { x: 0, y: 1 },
    Point { x: -1, y: 0 },
];

#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
struct Node {
    position: Point,
    path: Vec<Point>,
    length: u32
}

impl Node {
    fn score(&self, goal: &Point) -> u32 {
        let manhatten_distance = (goal.x - self.position.x).abs() as u32 + (goal.y - self.position.y).abs() as u32;
        manhatten_distance + self.length
    }

    fn children(&self) -> Vec<Node> {
        DIRECTIONS.iter().map(|direction| Node {
            position: Point { x: self.position.x + direction.x, y: self.position.y + direction.y },
            path: self.path.iter().cloned().chain(),
            length: self.length + 1,
        }).collect()
    }
}

fn pop_smallest(nodes: &mut Vec<Node>, goal: &Point) -> Node {
    let mut index = 0;
    let mut smallest_index = 0;
    let mut smallest_score = 0;
    for node in nodes.iter() {
        let score = node.score(goal);

        if index == 0 || score < smallest_score {
            smallest_index = index;
            smallest_score = score;
        }

        index += 1;
    }

    nodes.swap_remove(smallest_index)
}

struct Board {
    tiles: Vec<bool>,
    width: usize,
}

impl Board {
    fn new(width: usize, height: usize) -> Board {
        Board {
            tiles: vec![false; width * height],
            width: width,
        }
    }
}

impl From<String> for Board {
    fn from(s: String) -> Board {
        let width;
        match s.find('\n') {
            None => return Board::new(0, 0),
            Some(index) => width = index,
        }

        let mut new_board = Board {
            tiles: Vec::new(),
            width: width,
        };

        for c in s.chars() {
            if c == ' ' {
                new_board.tiles.push(false);
            } else if c == 'X' {
                new_board.tiles.push(true);
            }
        }

        new_board
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut board_string = String::new();
        let mut x = 0usize;
        for value in self.tiles.iter() {
            if *value {
                board_string.push('X');
            } else {
                board_string.push(' ');
            }

            x += 1;
            if x >= self.width {
                x = 0;
                board_string.push('\n');
            }
        }
        write!(f, "{}", board_string)
    }

    fn pathfind_step(&self, start: &Point, goal: &Point) -> Option<Direction> {
        let mut frontier =
    }
}

fn main() {
    let mut board_string = String::new();
    board_string.push_str("XXXXX\n");
    board_string.push_str("X   X\n");
    board_string.push_str("X X X\n");
    board_string.push_str("X   X\n");
    board_string.push_str("XXXXX");
    let test: Board = board_string.into();
    println!("{}", test);

    let path = Node {
        position: Point {
            x: 3,
            y: 3
        },
        direction: Direction::UP,
        length: 5,
    };
    let goal = Point { x: 5, y: 5 };
    let mut children = path.children();
    for child in children.iter() {
        println!("{} {}", child.position.x, child.position.y);
    }
    println!("---");
    let smol = pop_smallest(&mut children, &goal);
    println!("{} {}", smol.position.x, smol.position.y);
    println!("---");
    for child in children {
        println!("{} {}", child.position.x, child.position.y);
    }
}
