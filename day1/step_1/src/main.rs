use std::fs::File;
use std::io;
use std::io::{BufRead};
use std::path::Path;


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

fn open_and_read_inputfile(file_path: &str) -> io::Result<io::Lines<io::BufReader<File>>>
{
    println!("Went into the open and read input file function");
    let path = Path::new(&file_path);
    let display = path.display();
    println!("Created the file path {} ", display);


    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };
    println!("opened the file");

    Ok(io::BufReader::new(file).lines())
}
