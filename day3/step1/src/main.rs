use load::open_and_read_inputfile;
use ndarray::Axis;
use ndarray::Array;

fn main() {
    println!("Advent of code day 2");
    let res  = get_result();
    println!("res is : {}", res);
}


fn pretty_print(a: &ndarray::prelude::ArrayBase<ndarray::OwnedRepr<char>, ndarray::prelude::Dim<[usize; 2]>>) {
    for i in 0..139 {
        print!("line : {} ", i);
        for j in 0..140 {
            print!("{}", a[[i, j]]);
        }
        println!("");
    }
}


fn get_result() -> u32 {
    let mut a: ndarray::prelude::ArrayBase<ndarray::OwnedRepr<char>, ndarray::prelude::Dim<[usize; 2]>> = Array::from_shape_fn((140,140), |_| '\0');
    let mut i = 0;
    if let Ok(lines) = open_and_read_inputfile("src/input.txt") {
        println!("Ok for opening and reading the input file");
        for line in lines {
            if let Ok(_ip) = line {
                a = fill_line(&_ip, i, a);
            }
            i+=1;
        }
    }
    pretty_print(&a);
    let res = calculte_score(&a);
    return res;
}

fn calculte_score(a: &ndarray::prelude::ArrayBase<ndarray::OwnedRepr<char>, ndarray::prelude::Dim<[usize; 2]>>) -> u32 {
    let mut number = String::from("");
    let mut res = 0;
    let mut is_current_number_valid = false;
    for i in 0..140 {
        for j in 0..140 {
            if a[[i,j]].is_digit(10) && !is_current_number_valid{ 
                number.push(a[[i,j]]);
                is_current_number_valid = check_neighbors(a, i, j);
            }
            else if a[[i,j]].is_digit(10) {
                number.push(a[[i,j]]);
            }
            else {
                if is_current_number_valid {
                    res += number.parse::<u32>().unwrap();
                    println!("line {}: valid number : {} found at ({} {}) total res is : {} ",i, number, i, j,res); // println!("valid number : {}", number);
                }
                number = String::from("");
                is_current_number_valid = false;
            }
        }
        if is_current_number_valid {
            res += number.parse::<u32>().unwrap();
            number = String::from("");
            is_current_number_valid = false;
        }
    }
    return res;
}

fn check_neighbors(a: &ndarray::prelude::ArrayBase<ndarray::OwnedRepr<char>, ndarray::prelude::Dim<[usize; 2]>>, i: usize, j: usize) -> bool {
    if i == 0 {
        if j == 0 {
            return is_symbol(a[[i,j+1]]) || is_symbol(a[[i+1,j]]) || is_symbol(a[[i+1,j+1]]);
        }
        else if j == a.len_of(Axis(1)) - 1{
            return is_symbol(a[[i,j-1]]) || is_symbol(a[[i+1,j]]) || is_symbol(a[[i+1,j-1]]);
        }
        return is_symbol(a[[i,j-1]]) || is_symbol(a[[i,j+1]]) || is_symbol(a[[i+1,j-1]]) || is_symbol(a[[i+1,j]]) || is_symbol(a[[i+1,j+1]]);
    }
    else if i == a.len_of(Axis(0)) - 1 {
        if j == 0 {
            return is_symbol(a[[i,j+1]]) || is_symbol(a[[i-1,j]]) || is_symbol(a[[i-1,j+1]]);
        }
        else if j == a.len_of(Axis(1)) - 1 {
            return is_symbol(a[[i,j-1]]) || is_symbol(a[[i-1,j]]) || is_symbol(a[[i-1,j-1]]);
        }
        return is_symbol(a[[i,j-1]]) || is_symbol(a[[i,j+1]]) || is_symbol(a[[i-1,j-1]]) || is_symbol(a[[i-1,j]]) || is_symbol(a[[i-1,j+1]]);
    }
    else if j == 0 {
        return is_symbol(a[[i,j+1]]) || is_symbol(a[[i-1,j]]) || is_symbol(a[[i-1,j+1]]) || is_symbol(a[[i+1,j]]) || is_symbol(a[[i+1,j+1]]);
    }
    else if j == a.len_of(Axis(1)) - 1 {
        return is_symbol(a[[i,j-1]]) || is_symbol(a[[i-1,j]]) || is_symbol(a[[i-1,j-1]]) || is_symbol(a[[i+1,j]]) || is_symbol(a[[i+1,j-1]]);
    }
    else {
        return is_symbol(a[[i,j-1]]) || is_symbol(a[[i-1,j]]) || is_symbol(a[[i-1,j-1]]) || is_symbol(a[[i+1,j]]) || is_symbol(a[[i+1,j-1]]) || is_symbol(a[[i+1,j+1]]) || is_symbol(a[[i,j+1]]) || is_symbol(a[[i-1,j+1]]);
    }
}

fn is_symbol(character: char) -> bool {
    return !character.is_numeric() && character != '.';
}
fn  fill_line(line: &String, index: usize, mut array: ndarray::prelude::ArrayBase<ndarray::OwnedRepr<char>, ndarray::prelude::Dim<[usize; 2]>>) -> ndarray::prelude::ArrayBase<ndarray::OwnedRepr<char>, ndarray::prelude::Dim<[usize; 2]>> {
    //println!("{}", line);
    for i in 0..line.len() {
        array[[index,i]] = line.chars().nth(i).unwrap();
    }
    return array;
}