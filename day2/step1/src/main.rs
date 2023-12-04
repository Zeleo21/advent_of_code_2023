use load::open_and_read_inputfile;
use regex::Regex;

const MAX_RED_CUBES: u32 = 12;
const MAX_GREEN_CUBES: u32 = 13;
const MAX_BLUE_CUBES: u32 = 14;

fn main() {
    println!("Advent of code day 2");
    let mut res = 0;
    if let Ok(lines) = open_and_read_inputfile("src/input.txt") {
        println!("Ok for opening and reading the input file");
        for line in lines {
            if let Ok(_ip) = line {
                res += calculate_line_value(&_ip);
            }
        }
    }
    println!("res is : {}", res);
}

fn calculate_line_value(line: &String) -> u32 {
    if line == ""{
        return 0;
    }
    let mut split_line = line.split(":");
    let game_id = get_number_id(split_line.nth(0).unwrap());
    let sets = split_line.nth(0).unwrap().split(";");
    let game_valid = get_game_result(sets);
    if !game_valid {
        return 0;
    }
    return game_id;
}

fn get_game_result(sets: std::str::Split<'_,&str>) -> bool {
    let reg_cube = Regex::new(r"^\s*(?<number>\d+)\s+(?<cubeColor>blue|green|red)").unwrap();
    for set in sets {
        let single_cube = set.split(',');
        for cube in single_cube {
            let Some(result) = reg_cube.captures(cube) else {
                println!("no match!");
                return false;
            };
            if &result["cubeColor"] == "blue" {
                if result["number"].parse::<u32>().unwrap() > MAX_BLUE_CUBES{
                    return false;
                }
            }
            else if &result["cubeColor"] == "green" {
                if result["number"].parse::<u32>().unwrap() > MAX_GREEN_CUBES {
                    return false
                }
            }
            else if &result["cubeColor"] == "red" {
                if result["number"].parse::<u32>().unwrap() > MAX_RED_CUBES {
                    return false;
                }
            }
        }
    }
    return true;
}

fn get_number_id(id: &str) -> u32 {
    let re = Regex::new(r"Game (?<number>[0-9]+)").unwrap();
    let Some(caps) = re.captures(id) else {
        println!("no match!");
        return 0;
    };
    return caps["number"].parse::<u32>().unwrap();
}
