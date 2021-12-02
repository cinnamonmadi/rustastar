use std::process;
use std::io;

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone)]
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
        DIRECTIONS.iter().map(|direction| { 
            let next_point = Point { x: self.position.x + direction.x, y: self.position.y + direction.y };
            let mut new_path = self.path.clone();
            new_path.push(next_point);

            Node {
                position: next_point,
                path: new_path,
                length: self.length + 1,
            }
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
    height: usize,
}

impl Board {
    fn new(width: usize, height: usize) -> Board {
        Board {
            tiles: vec![false; width * height],
            width: width,
            height: height,
        }
    }

    fn pathfind(&self, start: &Point, goal: &Point) -> Option<Vec<Point>> {

        let mut frontier: Vec<Node> = vec![Node {
            position: Point { x: start.x, y: start.y },
            path: Vec::new(),
            length: 0
        }];
        let mut explored: Vec<Node> = Vec::new();

        loop {
            if frontier.is_empty() {
                println!("Pathfinding failed!");
                return None;
            }

            let smallest = pop_smallest(&mut frontier, goal);

            if smallest.position.x == goal.x && smallest.position.y == goal.y {
                return Some(smallest.path);
            }

            let children = smallest.children();
            explored.push(smallest);

            'child_loop: for child in children {
                if child.position.x < 0 || child.position.y < 0 || child.position.x >= self.width as i32 || child.position.y >= self.height as i32 {
                    continue 'child_loop;
                }

                let index = child.position.x as usize + (child.position.y as usize * self.width);
                if self.tiles[index] {
                    continue 'child_loop;
                }

                for node in explored.iter() {
                    if child.position.x == node.position.x && child.position.y == node.position.y {
                        continue 'child_loop;
                    }
                }

                for i in 0..frontier.len() {
                    if child.position.x == frontier[i].position.x && child.position.y == frontier[i].position.y {
                        if child.score(goal) < frontier[i].score(goal) {
                            frontier[i] = child;
                        }
                        continue 'child_loop;
                    }
                }

                frontier.push(child);
            }
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
            height: 0,
        };

        for c in s.chars() {
            if c == ' ' {
                new_board.tiles.push(false);
            } else if c == 'X' {
                new_board.tiles.push(true);
            }
        }

        new_board.height = new_board.tiles.len() / width;

        new_board
    }
}

fn print_board(board: &Board, pathfinder_position: &Point, goal_position: &Point) {
    for y in 0..board.height {
        for x in 0..board.width {
            if x as i32 == pathfinder_position.x && y as i32 == pathfinder_position.y {
                print!("O");
            } else if x as i32 == goal_position.x && y as i32 == goal_position.y { 
                print!("G");
            } else {
                let index = x + (y * board.width);
                if board.tiles[index] {
                    print!("X");
                } else {
                    print!(" ");
                }
            }
        }
        print!("\n");
    }
}

fn main() {
    let mut board_string = String::new();
    board_string.push_str("XXXXXXXXXXXXXXXX\n");
    board_string.push_str("X              X\n");
    board_string.push_str("X XXXXX   XXXXXX\n");
    board_string.push_str("X     X        X\n");
    board_string.push_str("X     X   XXX  X\n");
    board_string.push_str("X     X    X   X\n");
    board_string.push_str("X         X    X\n");
    board_string.push_str("X        X     X\n");
    board_string.push_str("XXXXXXXXXXXXXXXX");

    let board: Board = board_string.into();
    let start = Point { x: 5, y: 3 };
    let goal = Point { x: 11, y: 7 };
    let result = board.pathfind(&start, &goal);

    if result.is_none() {
        println!("Pathfinding failed! :(");
        process::exit(0);
    }

    let result = result.unwrap();
    let mut result_index = 0;

    while result_index != result.len() {
        print_board(&board, &result[result_index], &goal);

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line!");

        result_index += 1;
    }
}
