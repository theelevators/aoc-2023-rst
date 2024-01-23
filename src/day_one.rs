use std::usize;

pub fn solution_one(file: &str) {
    let mut sum: i32 = 0;

    for line in file.split("\n") {
        let line_nums: Vec<_> = line
            .chars()
            .filter(|c| c.to_string().parse::<i32>().is_ok())
            .collect();
        let num_count = line_nums.len();

        if num_count == 1 {
            let combo = format!("{}{}", line_nums.get(0).unwrap(), line_nums.get(0).unwrap());
            sum += combo.parse::<i32>().unwrap();
            continue;
        }

        if num_count >= 2 {
            let combo = format!(
                "{}{}",
                line_nums.first().unwrap(),
                line_nums.last().unwrap()
            );
            sum += combo.parse::<i32>().unwrap();
            continue;
        }
    }

    println!("{}", sum);
}

pub fn solution_two(file: &str) {
    let mut sum: i32 = 0;

    for line in file.split("\n") {
        let line_chars: Vec<String> = line.chars().map(|c| c.to_string()).collect();

        let mut word_num: String;
        let mut first: String;
        let mut start: u32 = 0;
        let mut end: u32 = 0;
        while end < line_chars.len() as u32 {
        
        let char:String = String::from(line_chars.get(start as usize).unwrap());
            
         if (start == 0) & (char.parse::<i32>().is_ok()) {
                first = char.to_string();   
                break;
            }
            let slice:&[String] = &line_chars[start as usize..end as usize];
    
            if is_number(slice){

                first = slice.join("");
                break;
            }
            
        }
        println!("first")
   }
}


fn is_number(slice:&[String])-> bool{

    match slice.join("").as_str() {
       "one" => true,
        "two"=> true,
        "three" => true,
        "four" => true,
        "five" => true,
        "six" => true,
        "seven" => true,
        "eight" => true,
        "nine" => true,
        _ => false
    }
}       
