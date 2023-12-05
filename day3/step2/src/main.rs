use load::open_and_read_inputfile;
use ndarray::Array;
use std::collections::VecDeque;
use ndarray::Axis;

struct Point {
    value : u32,
    i: u32,
    j: u32
}

struct Line {
    points : Vec<Point>
}


fn main() {
    println!("Advent of code day 2");
    let res  = get_result();
    println!("res is : {}", res);
}


fn pretty_print(a: &ndarray::prelude::ArrayBase<ndarray::OwnedRepr<char>, ndarray::prelude::Dim<[usize; 2]>>) {
    for i in 0..140 {
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
    let res = calculte_score(&a);
    return res;
}

fn calculte_score(a: &ndarray::prelude::ArrayBase<ndarray::OwnedRepr<char>, ndarray::prelude::Dim<[usize; 2]>>) -> u32 {
    
    let mut map: VecDeque<Line> =  VecDeque::new();

    map.push_back(get_line(&a, -1));
    map.push_back(get_line(&a, 0));
    map.push_back(get_line(&a, 1));

    for line in &map {
        pretty_print_line(&line);
        println!("------------------------------------");
    }

    let mut number = String::from("");
    let mut res = 0;
    for i in 0..140 {
        map.push_back(get_line(&a, i));
        map.push_back(get_line(&a, i + 1));
        map.push_back(get_line(&a, i - 1));
        for j in 0..140 {
            if a[[i as usize,j]] == '*' {
                
            }
        }
        //let mut line = get_line(&a, i);
    }
    return 0;
}

fn get_total_gear(map: &VecDeque<Line>, index: isize) -> u32 {
    //TODO
    return 0;
}


fn is_line_empty(line: &Line) -> bool {
    return line.points.len() == 0;
}
fn pretty_print_line(line: &Line) {
    if is_line_empty(line) {
        println!("line is empty");
    }
    for point in &line.points {
        println!("value : {}, i:{}, j:{}", point.value, point.i, point.j);
    }
}
 
fn get_line(a: &ndarray::prelude::ArrayBase<ndarray::OwnedRepr<char>, ndarray::prelude::Dim<[usize; 2]>>, index: isize) -> Line {
    let mut line = Line{points: Vec::new()};
    let mut number = String::from("");

    if index < 0 || index >= a.len() as isize {
        return line;   
    }
    for j in 0..a.len_of(Axis(1)) {

        if a[[index as usize,j]].is_digit(10) {
            number.push(a[[index as usize,j]]);
        }
        else {
             if number != "" {
                  //println!("{}", j);
                  line.points.push(Point{value: number.parse::<u32>().unwrap(), i: (j - number.len()) as u32, j: (j - 1) as u32});
             }
            number = String::from("");
            
        }
    }
    if number != "" {
        line.points.push(Point{value: number.parse::<u32>().unwrap(), i: (140 - number.len()) as u32, j: (139) as u32});
        number = String::from("");
    }
    return line;
}

fn  fill_line(line: &String, index: usize, mut array: ndarray::prelude::ArrayBase<ndarray::OwnedRepr<char>, ndarray::prelude::Dim<[usize; 2]>>) -> ndarray::prelude::ArrayBase<ndarray::OwnedRepr<char>, ndarray::prelude::Dim<[usize; 2]>> {
    //println!("{}", line);
    for i in 0..line.len() {
        array[[index,i]] = line.chars().nth(i).unwrap();
    }
    return array;
}