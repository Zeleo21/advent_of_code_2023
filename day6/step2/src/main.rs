use load::open_and_read_inputfile;
use regex::Regex;

struct Races {
    time: String,
    distance: String,
}
fn main() {
    println!("advent of code day 6!");
    let mut res = 0;
    let mut races = Races { time: String::new(), distance: String::new(), };
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
        let number = captures.as_str();
        races.time.push_str(number);
    }

    for captures in re.find_iter(distance_values) {
        let number = captures.as_str();
        races.distance.push_str(number);
    }
    println!("{}\n{}", races.time, races.distance);
}

fn get_result(races: &Races) -> u64 {
    return get_all_choices_for_race(races.time.parse::<u64>().unwrap(), races.distance.parse::<u64>().unwrap());
}

fn get_all_choices_for_race(time: u64, record_distance: u64) -> u64 {
    let mut res = 0;
    for holding_time in 0..time {
        if holding_time * (time - holding_time) > record_distance {
            res += 1;
        }
    }
    return res;
}