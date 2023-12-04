use load::open_and_read_inputfile;
use std::fmt::Debug;


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
    let mut res = 0;
    let mut a: ndarray::prelude::ArrayBase<ndarray::OwnedRepr<char>, ndarray::prelude::Dim<[usize; 2]>> = Array::from_shape_fn((139,140), |_| '\0');
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
    return res;
}

fn  fill_line(line: &String, index: usize, mut array: ndarray::prelude::ArrayBase<ndarray::OwnedRepr<char>, ndarray::prelude::Dim<[usize; 2]>>) -> ndarray::prelude::ArrayBase<ndarray::OwnedRepr<char>, ndarray::prelude::Dim<[usize; 2]>> {
    //println!("{}", line);
    for i in 0..line.len() {
        array[[index,i]] = line.chars().nth(i).unwrap();
    }
    return array;
}