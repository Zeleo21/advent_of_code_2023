use std::collections::{BTreeMap, HashMap};

use load::open_and_read_inputfile;
use regex::Regex;


#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
struct Hand {
    hand: String,
    bid: u32,
    hand_type: u32,
}

#[derive(Debug, Clone)]
struct Game {
    map: BTreeMap<Hand, u32>
}

fn main() {
    println!("advent of code day 6!");
    let mut res = 0;
    let mut game = Game {map: BTreeMap::new()};

    // let test_hand = Hand {hand: String::new(), bid: 32, hand_type: String::new()};
    // game.map.insert(1, test_hand);

    // for (key, value) in &game.map { 
    //     println!("Key: {}, value: {}", key, value.bid);
    // }

    if let Ok(lines) = open_and_read_inputfile("src/input.txt") {
        for (count,line)  in lines.enumerate() {
            if let Ok(_ip) = line {
                insert_hand(&_ip, &mut game)
            }
        }
    }
    
    for(key,value) in game.map.clone().into_iter() {
        println!("key: {}, value: {}", key.hand, value);
    }
        println!("total res is: {}", res);
}

fn insert_hand(line: &String, game: &mut Game){
    let mut splited_line = line.split(" ");
    let hand = splited_line.nth(0).unwrap();
    let bid = splited_line.nth(0).unwrap().parse().unwrap();
    let hand_type = get_type_of_hand(&hand.to_owned());

    let newHand = Hand{hand: hand.to_owned(), bid: bid, hand_type: hand_type};
    println!("new hand is: {}", newHand.hand);
    update_game(game, &newHand);
}

fn update_game(game: &mut Game, hand: &Hand) {
    let mut rank = 0;
    if game.map.len() == 0 {
        game.map.insert(hand.clone(),1);
        return;
    }
    for (key,value) in game.map.clone().into_iter() {
        if compare_hand(&key, &hand) && rank == 0 {
            println!("NEWHAND IS HIGHER");
            continue;
        } 
        else if rank == 0 {
            rank = value;
            println!("INSERTING");
            game.map.insert(hand.clone(), rank);
        }
        else {
            *game.map.get_mut(&key).unwrap() += 1;
        }
    }
    if rank == 0 {
        game.map.insert(hand.clone(), rank);
    }
}

//return true if newHand should be higher in the rank than hand.
fn compare_hand(hand: &Hand, newHand: &Hand) -> bool {
    if newHand.hand_type > hand.hand_type {
        return true;
    }
    return compare_hand_string(&hand.hand, &newHand.hand);
}

fn compare_hand_string(hand: &String, newHand: &String) -> bool { 
    for i in 0..newHand.len() {
        if hand.chars().nth(i).unwrap() == newHand.chars().nth(i).unwrap() {
            continue;
        }
        return is_better_char(hand.chars().nth(i).unwrap(), newHand.chars().nth(i).unwrap());
    }
    return false;
}

fn is_better_char(first: char, second: char) -> bool {
    let order = "AKQJT98765432";
    return order.find(first) > order.find(second);
}

fn get_type_of_hand(hand: &String) -> u32 {
    let mut res = 0;
    let mut map: HashMap<char, u32> = HashMap::new();
    for (i, hand) in hand.chars().enumerate() {
        if !map.contains_key(&hand) {
            map.insert(hand, 1);
        }
        else {
            *map.get_mut(&hand).unwrap() += 1;
        }
    }
    for value in map.values() { 
        if *value == 5 && map.len() == 1{
            res = 7;
            break;
        }
        else if *value == 4 && map.len() == 2 {
            res = 6;
            break;
        }
        else if *value == 3 && map.len() == 2 {
            res = 5;
            break;
        }
        else if *value == 3 && map.len() == 3{
            res = 4;
            break;
        }
        else if *value == 2 && map.len() == 3{
            res = 3;
            break;
        }
        else if *value == 2 && map.len() == 4 {
            res = 2;
            break;
        }
        else if *value == 1 && map.len() == 5 {
            res = 1;
            break;
        }
    }   
    return res;
}