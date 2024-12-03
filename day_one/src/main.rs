use std::collections::HashMap;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn main() -> std::io::Result<()> {
    day_one()?;
    day_two()?;

    Ok(())
}

fn day_one() -> std::io::Result<()> {
    let f: File = File::open("data.txt")?;
    let reader = BufReader::new(f);

    let mut line_counter: i32 = 1;
    let mut left_sets: Vec<i64> = Vec::new();
    let mut right_sets: Vec<i64> = Vec::new();

    let mut total_distance: i64 = 0;

    for line in reader.lines() {
      line_counter += 1;
      println!("Line #:{}", line_counter);
      let line = line.unwrap();
      let mut sets_of_numbers = line.split("   ");
      let first_set = sets_of_numbers.next().unwrap().parse::<i64>().unwrap();
      let second_set = sets_of_numbers.next().unwrap().parse::<i64>().unwrap();

      left_sets.push(first_set);
      right_sets.push(second_set);
    }

    left_sets.sort();
    right_sets.sort();

    // Check if number of sets are the same
    if left_sets.len() != right_sets.len() {
      println!("Not the same length");
      return Ok(());
    }

    // go through each set and get the distance between each digit
    for i in 0..left_sets.len() {
      let current_left_set: i64 = left_sets[i];
      let current_right_set: i64 = right_sets[i];

      // get the distance between the two, add it to totalDistance
      let distance: i64 = (current_left_set - current_right_set).abs();

      total_distance += distance;
    }

    println!("Total distance: {}", total_distance);

    Ok(())
}

fn day_two() -> std::io::Result<()> {
    println!("Day 2");
    // Find the similiarity score between the two lists
    let f: File = File::open("data.txt")?;
    let reader = BufReader::new(f);

    // we can do a hashmap, where the key is the numbers in the left list and the value is the count (number of times it appears on the right list)\
    let mut numbers_map: HashMap<i64, i64> = HashMap::new();
    let mut right_numbers: Vec<i64> = Vec::new();
    
    for line in reader.lines() {
      let line = line.unwrap();
      let mut sets_of_numbers = line.split("   ");
      let first_set = sets_of_numbers.next().unwrap().parse::<i64>().unwrap();
      numbers_map.insert(first_set, 0);
      let second_set = sets_of_numbers.next().unwrap().parse::<i64>().unwrap();
      
      right_numbers.push(second_set);
    }

    for num in right_numbers {
      // check if number is key in the hashmap
      if numbers_map.contains_key(&num) {
        let count: &i64 = numbers_map.get(&num).unwrap();
        numbers_map.insert(num, count + 1);
      }
    }

    // finally get the sum of all the values in the hashmap
    let mut sum: i64 = 0;
    for (key, value) in numbers_map.iter() {
      sum += key * value;
    }
    
    println!("Similarity Score: {}", sum);

    Ok(())
}