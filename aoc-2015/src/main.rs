use std::fs;

mod days;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() <= 1 {
        println!("No argument provided");
    } else {
        for arg in &args[1..] {
            let data: String = fs::read_to_string(format!("src/input/{}.txt", arg)).unwrap();
            let mut res1 = 0;
            let mut res2 = 0;
            match arg.as_str() {
                "1" => {
                    res1 = days::one::task_one(&data);
                    res2 = days::one::task_two(&data);
                }
                "2" => {
                    res1 = days::two::task_one(&data);
                    res2 = days::two::task_two(&data);
                }
                "3" => {
                    res1 = days::three::task_one(&data);
                    res2 = days::three::task_two(&data);
                }
                "4" => {
                    println!("Day 4 implemented in Python!");
                    continue;
                }
                "5" => {
                    res1 = days::five::task_one(&data);
                    res2 = days::five::task_two(&data);
                }
                _ => {}
            }
            println!("Result for day {arg} task 1: {res1}");
            println!("Result for day {arg} task 2: {res2}");
        }
    }
}
