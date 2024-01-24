

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

        if line == "" {continue};
        let line_chars: Vec<String> = line.chars().map(|c| c.to_string()).collect();
        let mut reverse_chars:Vec<String> = line_chars.clone();
        reverse_chars.reverse();
        let mut first:String = String::new();
        let mut last: String = String::new();
        let mut idx = 0;
        loop {
            let current: String = String::from(line_chars.get(idx).unwrap());

            if current.parse::<i32>().is_ok() {
                first = current;
                break;
            }
            let slice: &[String] = &line_chars[0..idx+1];
            
            if is_number(slice) {
                first = to_number(slice).to_string();
                break;
            }

            let leftover = has_number(&slice);

            if leftover.0 {

                first = leftover.1;
                break;
            }

            if idx == line_chars.len() {
                
                idx = 0;
                continue;
            }
            idx += 1;
        }

        idx = 0;
        loop {
            let current: String = String::from(reverse_chars.get(idx).unwrap());
            
            if current.parse::<i32>().is_ok() {
                last = current;
                break;
            }
            let mut slice:Vec<String> = reverse_chars[0..idx+1].iter().map(|c| c.to_string()).collect();
                slice.reverse();
           
            if is_number(&slice) {
                last = to_number(&slice).to_string();
                break;
            }
            
            let leftover = reverse_has_number(&slice);

            if leftover.0 {
                last = leftover.1;
                break;
            }
            if idx == reverse_chars.len() {
                idx = 0;
                continue;
            }
            idx += 1;
        }

        let combo = format!("{}{}",first,last);
        println!("{combo}");
        sum += combo.parse::<i32>().unwrap();
    }
    println!("{}",sum);
}

fn reverse_has_number(slice: &[String])->(bool,String){
    let mut slice = Vec::from(slice);
    loop {
        if slice.len() == 0 {
                return (false, "".to_string());
            }

        slice.reverse();
        slice.pop();
        slice.reverse();
        println!("{:?}", slice);
        if is_number(&slice){
            return (true, slice.join(""));
        }

    }
}



fn has_number(slice: &[String])->(bool,String){
   let mut slice = Vec::from(slice);

    loop{
        if slice.len() == 0{
            return (false, "".to_string());
        }

        slice.pop();
        if is_number(&slice){
            return (true, slice.join(""));
        }
        
    }
}
fn to_number(slice: &[String]) -> i32{
    match slice.join("").as_str() {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => panic!("number not supported"),
    }

}

fn is_number(slice: &[String]) -> bool {

    
    

    match slice.join("").as_str() {
        "one" => true,
        "two" => true,
        "three" => true,
        "four" => true,
        "five" => true,
        "six" => true,
        "seven" => true,
        "eight" => true,
        "nine" => true,
        _ => false,
    }
}
