use load::open_and_read_inputfile;
use std::collections::HashMap;
use regex::Regex;

fn main() {
    println!("advent of code day 4!");
    let mut map : HashMap<u32, u32> = HashMap::new();

    if let Ok(lines) = open_and_read_inputfile("src/input.txt") {
        println!("Ok for opening and reading the input file");
        for line in lines {
            if let Ok(_ip) = line {
                get_result(&_ip, &mut map);
            }
        }
    }
    let mut res = 0;
    for i in 1..map.len() {
        let mut value = map.get(&(i as u32)).unwrap();
        res += value;
        println!("card: {}, copies: {}, total res is: {}", i, value, res);
    }
}

fn get_result(line: &String, map: &mut HashMap<u32, u32>) {
    let mut game = line.split(':');
    let card_id = game.nth(0).unwrap();
    let regex_number = Regex::new(r"Card\s+(?<number>[0-9]+$)").unwrap();
    let Some(card_id) = regex_number.captures(card_id) else { 
        println!("No match found");
        return;
    };
    let card_id = &card_id["number"];

    let mut hands = game.nth(0).unwrap().split("|");

    let winning_hand = hands.nth(0).unwrap();
    let current_hand = hands.nth(0).unwrap();
    let matching_numbers = get_matches(&current_hand.to_owned(), &winning_hand.to_owned());
    update_map(map, card_id, matching_numbers);
}
fn update_map(map: &mut HashMap<u32, u32>, card_id: &str, copies: u32) {
    let mut update_id = card_id.parse::<u32>().unwrap();
    if copies == 0 && !map.contains_key(&update_id) {
        map.insert(update_id, 1);
    }
    let mut increment = 1;
    if map.contains_key(&update_id) {
        increment = *map.get(&update_id).unwrap();
    }
    for _i in 0..copies {
        //check if key exists in map
        if !map.contains_key(&update_id) {
            map.insert(update_id, 1);
        }
        if !map.contains_key(&(update_id + 1)) {
            map.insert(update_id + 1, 1);
        }
        *map.get_mut(&(update_id + 1)).unwrap() += increment;
        update_id += 1;
    }
}
fn get_matches(current_hand: &String, winning_hand: &String) -> u32 {
    let mut result = 0;
    let current_cards = current_hand.split(' ');
    let winning_cards = winning_hand.split(' ');
    for c in current_cards {
        for w in winning_cards.clone() {
            if c == w && c != "" {
                result += 1;
            }
        }
    }
    return result;
}
