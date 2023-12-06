use load::open_and_read_inputfile;

fn main() {
    println!("advent of code day 4!");
    let mut res = 0;
    if let Ok(lines) = open_and_read_inputfile("src/input.txt") {
        println!("Ok for opening and reading the input file");
        for line in lines {
            if let Ok(_ip) = line {
                res += get_result(&_ip);
            }
        }
    }
    println!("result: {}", res);
}

fn get_result(line: &String) -> u32 {
    let mut result = 0;
    let mut game = line.split(':').nth(1).unwrap().split('|');
    let winning_hand = game.nth(0).unwrap();
    let current_hand = game.nth(0).unwrap();
    result += get_matches(&current_hand.to_owned(), &winning_hand.to_owned());
    return result;
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
    return get_final_result(result)
}

fn get_final_result(multiplier: u32) -> u32 {
    let mut result = 0;
    for _i in 0..multiplier {
        if result == 0 {
            result = 1;
        }
        else {
            result *= 2;
        }
    }
    return result;
}
