use regex::Regex;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn main() {
    println!("Hello, world!");
    let f: File = std::fs::File::open("data.txt").expect("Could not open input file");
    let reader: BufReader<File> = BufReader::new(f);
    let mut total_sum: i64 = 0;

    for line in reader.lines() {
        let line: String = line.unwrap();
        let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
        for (_, [x, y]) in re.captures_iter(&line).map(|x| x.extract()){
            let x_value: i64 = x.parse::<i64>().unwrap();
            let y_value: i64 = y.parse::<i64>().unwrap();
            let mult_value = x_value * y_value;
            total_sum += mult_value;
            println!("{} * {} = {} | Total: {}", x, y, mult_value, total_sum);
        }
        
        println!("Total Sum: {}", total_sum);
    }
}
