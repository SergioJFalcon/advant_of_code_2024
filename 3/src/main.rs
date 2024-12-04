use regex::Regex;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn main() {
    println!("Hello, world!");
    // part_one();
    part_two();
}

fn _part_one() {
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

fn part_two() {
    let f: File = std::fs::File::open("data.txt").expect("Could not open input file");
    let reader: BufReader<File> = BufReader::new(f);
    let mut enables_mul_instructions: bool = true;
    let mut total_sum: i64 = 0;

    for line in reader.lines() {
        let line: String = line.unwrap();

        // TODO: 
        if enables_mul_instructions {
          let up_to_first_do: Regex = Regex::new(r"^(.*?)(?:do\(\))").unwrap();
  
          for (_, [enabled_instructions]) in up_to_first_do.captures_iter(&line).map(|x| x.extract()){
            println!("Enabled Instructions: {}", enabled_instructions);
            let re: Regex = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
            for (_, [x, y]) in re.captures_iter(&enabled_instructions).map(|x| x.extract()){
                let x_value: i64 = x.parse::<i64>().unwrap();
                let y_value: i64 = y.parse::<i64>().unwrap();
                let mult_value: i64 = x_value * y_value;
                total_sum += mult_value;
                println!("\t\t{} * {} = {} | Total: {}", x, y, mult_value, total_sum);
            }
          }
        }
        // we can only capture the mul() function calls in the input file if its within a do() block, anything after a don't block is ignored
        
        // Regex has 3 groups but captures only the 2nd group which contains the mul() functions that we are going to parse and total
        // 1. (?:do\(\)) - matches the do()
        // 2. (.*?) - matches anything within the do() and  don't() or end of line
        // 3. (?:don't\(\)|$) - matches don't() or end of line
        let re_enabled: Regex = Regex::new(r"(?:do\(\))(.*?)(don't\(\)|$)").unwrap();
        for (_, [enabled_instructions, end_instruction]) in re_enabled.captures_iter(&line).map(|x| x.extract()){
            println!("Enabled Instructions: {}", enabled_instructions);
            let re: Regex = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
            for (_, [x, y]) in re.captures_iter(&enabled_instructions).map(|x| x.extract()){
                let x_value: i64 = x.parse::<i64>().unwrap();
                let y_value: i64 = y.parse::<i64>().unwrap();
                let mult_value: i64 = x_value * y_value;
                total_sum += mult_value;
                println!("\t\t{} * {} = {} | Total: {}", x, y, mult_value, total_sum);
            }
            println!("End Instruction: {}", end_instruction);
            if end_instruction == "don't()" {
              enables_mul_instructions = false;
            } else {
              enables_mul_instructions = true;
            }
        }

        println!("Total Sum: {}", total_sum);
    }
}