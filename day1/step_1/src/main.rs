use load::open_and_read_inputfile;

fn main() {
    println!("Advent of code day 1 !");
    let mut res = 0;
    if let Ok(lines) = open_and_read_inputfile("input/step1.txt") {
        println!("Ok for opening and reading the input file");
        for line in lines {
            if let Ok(_ip) = line {
                let mut value = calculate_line(&_ip);
                println!("value is {}", value);
                res += value;
            }
        }
    }
    println!("Final result is : {}", res);
}

fn calculate_line(line: &String) -> u32 {
    let mut res = 0;
    let mut last = 0;
    let mut first = 0;
    for char in line.chars() {
        if char.is_digit(10) && first == 0 {
            first = char.to_digit(10).unwrap();
            res += first;
        } else if char.is_digit(10) {
            last = char.to_digit(10).unwrap();
        }
    }
    res *= 10;
    if last == 0 {
        res += first;
    } else {
        res += last;
    }

    return res;
}