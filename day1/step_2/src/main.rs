use std::fs::File;
use std::io;
use std::io::{BufRead};
use std::path::Path;


fn main() {
    println!("Advent of code day 1 !");
    let mut res = 0;
    let mut value = 0;
    let mut newString = String::from("eighthree");
    res = calculate_line(&newString);
    println!("result is : {0}", res);
    //let s = String::from("two1nine");
   // println!("{}", check_beginning_of_number(s.as_str().chars().nth(2).unwrap())); // true
   
    if let Ok(lines) = open_and_read_inputfile("input/step2.txt") {
        println!("Ok for opening and reading the input file");
        for line in lines {
            if let Ok(_ip) = line {
                value = calculate_line(&_ip);
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
    let mut i = 0;
    let mut tmp;
    let mut index;
    while i < line.len() {
        tmp = line.chars().nth(i).unwrap();
        if tmp.is_digit(10) && first == 0 {
            first = tmp.to_digit(10).unwrap();
            res += first;
        } else if tmp.is_digit(10) {
            last = tmp.to_digit(10).unwrap();
        }
        else if first == 0 && check_beginning_of_number(tmp) {
            (first, index) = get_number_from_string(line, i as u32);
            println!("index is : {0} and first value is : {1}", index, first);
            res += first;
            i += index as usize;
        }
        else if check_beginning_of_number(tmp) {
            let mut temporary;
            (temporary, index) = get_number_from_string(line, i as u32);
            if temporary != 0 {
                last = temporary;
            }
            println!("index is : {0} and last value is : {1}", index, last);
            i+= index as usize;
        }
            i+=1;
        println!("value of i is : {0}", i);
    }
    res *= 10;
    if last == 0 {
        res += first;
    } else {
        res += last;
    }

    return res;
}

fn get_number_from_string(line: &String, index: u32) -> (u32,u32) {

    if index as usize >= line.len() {
        println!("Superior to the length of the line");
        return (0,0);
    }

    let mut i = index as usize;
    let mut res : String = String::from("");
    let mut value : u32 = 0;
    while i < line.len() {
        res.push(line.chars().nth(i).unwrap());
        //println!("{}", res);
        value = return_number_from_string(&res);
        //println!("value: {}", value);
        if value != 0 {
            return (value,(i - index as usize) as u32);
        }
        i+=1;
    }
    //println!("No number in the line");
    return (0,0);
}

fn return_number_from_string(line: &String) -> u32 {
    match line.as_str() {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => 0
    }
}

fn check_beginning_of_number(character: char) -> bool {
    return character == 'o' || character == 't' || character == 'f' || character == 's'
        || character == 'e' || character == 'n';
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
