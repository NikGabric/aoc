use std::collections::HashSet;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new() -> Self {
        Self { x: 0, y: 0 }
    }
    fn move_right(&mut self) {
        self.x += 1;
    }
    fn move_down(&mut self) {
        self.y -= 1;
    }
    fn move_left(&mut self) {
        self.x -= 1;
    }
    fn move_up(&mut self) {
        self.y += 1;
    }
}

pub fn task_one(data: &str) -> i32 {
    let mut visited: HashSet<Point> = HashSet::new();
    visited.insert(Point::new());
    let mut location = Point::new();
    for c in data.chars() {
        match c {
            '>' => location.move_right(),
            'v' => location.move_down(),
            '<' => location.move_left(),
            '^' => location.move_up(),
            _ => {}
        }
        visited.insert(location);
    }
    visited.len() as i32
}

#[test]
fn test_one() {
    let input = "^v^v^v^v^v";
    assert_eq!(task_one(input), 2);
}

pub fn task_two(data: &str) -> i32 {
    let mut visited: HashSet<Point> = HashSet::new();
    visited.insert(Point::new());
    let mut location = Point::new();
    let mut robo_location = Point::new();
    for (i, c) in data.chars().enumerate() {
        match i % 2 {
            0 => {
                match c {
                    '>' => location.move_right(),
                    'v' => location.move_down(),
                    '<' => location.move_left(),
                    '^' => location.move_up(),
                    _ => {}
                }
                visited.insert(location);
            }
            1 => {
                match c {
                    '>' => robo_location.move_right(),
                    'v' => robo_location.move_down(),
                    '<' => robo_location.move_left(),
                    '^' => robo_location.move_up(),
                    _ => {}
                }
                visited.insert(robo_location);
            }
            _ => {}
        }
    }
    visited.len() as i32
}

#[test]
fn test_two() {
    let input = "^v^v^v^v^v";
    assert_eq!(task_two(input), 11);
}
