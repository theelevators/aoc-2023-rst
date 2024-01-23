
pub fn solution_one(file:&str) {
    
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



pub fn solution_two(file:&str){

    let mut sum:i32 = 0;
    
     for line in file.split("\n") {

        let line_chars:Vec<String> = line.chars().map(|c| c.to_string()).collect();
    
    
        if let (Some(first), Some(last)) =  (line_chars.first().unwrap().parse::<i32>().ok() , line_chars.last().unwrap().parse::<i32>().ok()) {
                let combo = format!("{}{}",first,last);

                sum += combo.parse::<i32>().unwrap();
            continue;
        } 

        
        
         
    }
}
