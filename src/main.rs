use std::fs;
mod day_one;

fn main() {
    
    let file: String = fs::read_to_string("./day_one.txt").unwrap();
    let test_file: String = fs::read_to_string("./test_two.txt").unwrap();
    day_one::solution_one(&file);
    day_one::solution_two(&test_file);
}
