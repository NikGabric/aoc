pub fn task_one(data: &str) -> i32 {
    let mut floor: i32 = 0;
    data
        .chars()
        .for_each(|c| {
            match c {
                '(' => floor += 1,
                ')' => floor -= 1,
                _ => {}
            }
        });
    floor
}

#[test]
fn test_one() {
    let input = "))(((((";
    assert_eq!(task_one(input), 3);
}

pub fn task_two(data: &String) -> i32 {
    let mut floor: i32 = 0;
    let mut index: i32 = 0;
    for (i, c) in data.chars().enumerate() {
        match c {
            '(' => floor += 1,
            ')' => {
                floor -= 1;
                if floor <= -1 {
                    index = (i + 1) as i32;
                    break;
                }
            },
            _ => {}
        }
    }
    index
}
