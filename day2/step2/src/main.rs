use load::open_and_read_inputfile;
use regex::Regex;

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
    let mut split_line = line.split(":");
    let sets = split_line.nth(1).unwrap().split(";");
    let game_valid = get_game_result(sets);
    return game_valid;
}

fn get_game_result(sets: std::str::Split<'_,&str>) -> u32 {
    let reg_cube = Regex::new(r"^\s*(?<number>\d+)\s+(?<cubeColor>blue|green|red)").unwrap();
    let mut red = 0;
    let mut blue = 0;
    let mut green = 0;
    for set in sets {
        let single_cube = set.split(',');
        for cube in single_cube {
            let Some(result) = reg_cube.captures(cube) else {
                println!("no match!");
                return 0;
            };
            if &result["cubeColor"] == "blue" && result["number"].parse::<u32>().unwrap() > blue{
                    blue = result["number"].parse::<u32>().unwrap();
            }
            else if &result["cubeColor"] == "green" && result["number"].parse::<u32>().unwrap() > green {
                    green = result["number"].parse::<u32>().unwrap();
            }
            else if &result["cubeColor"] == "red" && result["number"].parse::<u32>().unwrap() > red{
                    red = result["number"].parse::<u32>().unwrap();
            }
        }
    }
    return red * blue * green;
}

