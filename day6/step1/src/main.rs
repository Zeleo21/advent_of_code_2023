use load::open_and_read_inputfile;
use regex::Regex;
struct Races {
    time: Vec<u32>,
    distance: Vec<u32>,
}
fn main() {
    println!("advent of code day 6!");
    let mut res = 0;
    let mut races = Races { time: Vec::new(), distance: Vec::new()};
    if let Ok(lines) = open_and_read_inputfile("src/input.txt") {
        println!("Ok for opening and reading the input file");
        let mut time = String::new();
        let mut distance = String::new();
        for (count,line)  in lines.enumerate() {
            if let Ok(_ip) = line {
                if count == 0 {
                    time = _ip;
                }
                else {
                    distance = _ip;
                }
            }
        }
        fill_races(&mut races, &time, &distance);
        res = get_result(&races);
    }
    println!("total result is : {}", res);
}

fn fill_races(races: &mut Races, time: &String, distance: &String) {
    let time_values = time.split(':').nth(1).unwrap();
    let distance_values = distance.split(':').nth(1).unwrap();

    let re = Regex::new(r"\d+").unwrap();
    for captures in re.find_iter(time_values) {
        let number: usize = captures.as_str().parse().unwrap();
        races.time.push(number as u32);
    }

    for captures in re.find_iter(distance_values) {
        let number: usize = captures.as_str().parse().unwrap();
        races.distance.push(number as u32);
    }
}

fn get_result(races: &Races) -> u32 {
    let mut res = 1;
    for i in 0..races.time.len() {
        res *= get_all_choices_for_race(races.time[i], races.distance[i]);
    }
    return res;
}

fn get_all_choices_for_race(time: u32, record_distance: u32) -> u32 {
    let mut res = 0;
    for holding_time in 0..time {
        if holding_time * (time - holding_time) > record_distance {
            res += 1;
        }
    }
    return res;
}