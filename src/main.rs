use std::fs;
mod day_one;

fn main() {
    
    let file: String = fs::read_to_string("./day_one.txt").unwrap();
    day_one::solution_one(&file);
    day_one::solution_two(&file);
}
