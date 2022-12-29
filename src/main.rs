use std::env;
mod search;
mod day1;
mod day2;
mod day19;
mod twod;

fn main() {
    let args: Vec<String> = env::args().collect();

    match &args[1][..] {
        "day1" | "1" => day1::run(),
        "day2" | "2" => day2::run(),
        "day19" | "19" => day19::run(),
        _ => println!("No such day"),
    }
}
