use std::fs::File;
use std::io;
use std::io::{BufRead};
use std::path::Path;

pub fn open_and_read_inputfile(file_path: &str) -> io::Result<io::Lines<io::BufReader<File>>>
{
    let path = Path::new(&file_path);
    let display = path.display();


    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    Ok(io::BufReader::new(file).lines())
}