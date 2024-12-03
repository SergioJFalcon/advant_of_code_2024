use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn main() -> std::io::Result<()> {
    println!("Hello, world!");
    let f = File::open("data.txt")?;
    let mut reader = BufReader::new(f);
    let mut totalDistance: i32 = 0;
    
    let mut line = String::new();
    let len = reader.read_line(&mut line).unwrap();
    println!("First line is {} bytes long", len);
    println!("{}", line);
    
    // Line will always have two sets of numbers separated by 3 spaces
    let mut sets_of_numbers = line.split("   ");
    // split line in two delimited by 3 spaces, then parse the numbers into 2 vectors
    let mut first_set = sets_of_numbers.next().unwrap();
    let mut second_set = sets_of_numbers.next().unwrap();
    println!("first_set: {}", first_set);
    println!("second_set: {}", second_set);
    let example = "123456";

    let mut first_number: Vec<u32> = first_set.chars().map(|c| c.to_digit(10).unwrap()).collect();
    println!("{:?}", first_number);
    let mut line_counter = 1;
    for line in reader.lines() {
        line_counter += 1;
        let line = line.unwrap();
        let mut sets_of_numbers = line.split("   ");
        let mut first_set = sets_of_numbers.next().unwrap();
        let mut second_set = sets_of_numbers.next().unwrap();
        let mut first_list: Vec<u32> = first_set.chars().map(|c| c.to_digit(10).unwrap()).collect();
        let mut second_list: Vec<u32> = second_set.chars().map(|c| c.to_digit(10).unwrap()).collect();
        
        first_list.sort();
        second_list.sort();
        println!("Line #:{}", line_counter);
        println!("Line: {}", line);
        println!("\t{:?}", first_list);
        println!("\t{:?}", second_list);
        if first_list.len() != second_list.len() {
            println!("Not the same length");
            continue;
        } else {
          let mut distance_of_pairs: i32= 0;
          for i in 0..first_list.len() {
            // get the distance between the two, add it to totalDistance
            let distance: i32 = (first_list[i] as i32 - second_list[i] as i32).abs();
            println!("\t\tDistance: {}\n", distance);
            distance_of_pairs += distance;
          }
          println!("\t\tDistance of pairs: {}\n", distance_of_pairs);
          totalDistance += distance_of_pairs;
          println!("\t\t\tTotal distance: {}\n", totalDistance);
        }
    }
    println!("Total distance: {}", totalDistance);
    Ok(())
}
