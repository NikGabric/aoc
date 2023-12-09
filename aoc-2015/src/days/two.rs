pub fn task_one(data: &str) -> i32 {
    let mut paper = 0;
    for line in data.lines() {
        let mut dim: Vec<i32> = line
            .split("x")
            .map(|c| c.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        dim.sort();
        paper += 2 * dim[0] * dim[1] + 2 * dim[1] * dim[2] + 2 * dim[0] * dim[2];
        paper += dim[0] * dim[1];
    }
    paper
}

#[test]
fn test_one() {
    let input = "2x3x4";
    assert_eq!(task_one(input), 58);
}

pub fn task_two(data: &str) -> i32 {
    let mut ribbon = 0;
    for line in data.lines() {
        let mut dim: Vec<i32> = line
            .split("x")
            .map(|c| c.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        dim.sort();
        ribbon += 2 * dim[0] + 2 * dim[1] + dim[0] * dim[1] * dim[2];
    }
    ribbon
}

#[test]
fn test_two() {
    let input = "2x3x4";
    assert_eq!(task_two(input), 34);
}
