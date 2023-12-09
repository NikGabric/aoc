use std::fs;

mod days;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() <= 1 {
        println!("No argument provided");
    } else {
        for arg in &args[1..] {
            let data: String = fs::read_to_string(format!("src/input/{}.txt", arg)).unwrap();
            let mut res1= 0;
            let mut res2= 0;
            match arg.as_str() {
                 "1" => {
                    res1 = days::one::task_one(&data);
                    res2 = days::one::task_two(&data);
                }
                _ => {}
            }
            println!("Result of task one: {res1}");
            println!("Result of task two: {res2}");
        }
    }
}
