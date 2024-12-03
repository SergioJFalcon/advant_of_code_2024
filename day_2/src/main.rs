use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn main() -> std::io::Result<()> {
    let f: File = std::fs::File::open("example.txt")?;
    let reader: BufReader<File> = BufReader::new(f);

    let mut safe_count: i32 = 0;
    let mut unsafe_count: i32 = 0;

    for line in reader.lines() {
        let line: String = line.unwrap();
        // Split the line into a vector of numbers
        let numbers: Vec<i32> = line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
        // must be increasing or decreasing in intervals of 1 or 3 to be safe, otherwise its unsafe
        // Check if the values in each line are either decreasing or increasing, if nots its unsafe
        let is_seq_safe: bool = check_for_safe_sequence(numbers.clone(), 0);
        
        
        if is_seq_safe {
          safe_count += 1;
          // println!("Current safe_count: {}", safe_count);
        } else {
          unsafe_count += 1;
          // println!("\n\tXXX - {:?} - {}", numbers, is_seq_safe);
        }
    }

    println!("\n\nSafe: {}", safe_count);
    println!("Unsafe: {}", unsafe_count);
    Ok(())
}


fn check_for_safe_sequence(mut set_of_nums: Vec<i32>, mut problems_counter: i32) -> bool {
    // Check the set if its safe
    let mut is_safe: bool = true;
    let mut is_increasing: bool = false;
    let mut problem_index: usize = 0;


    // if problems_counter > 0 {
    //   println!("\t\t{:?} - counter: {}", set_of_nums, problems_counter);
    // }

    if problems_counter > 1 {
      // println!("\n\t\t*****{:?} - counter: {}*****\n", set_of_nums, problems_counter);
      return false;
    }

    for i in 0..set_of_nums.len() {
        if i == 0 {
          continue;
        }

        if i == 1 {
          // Check if the first two numbers are either increasing or decreasing, follow that pattern for the rest of the vector of numbers
          if set_of_nums[i] > set_of_nums[i - 1] {
            is_increasing = true;
          } 
          if set_of_nums[i] < set_of_nums[i - 1] {
            is_increasing = false;
          }
        }
        
        // println!("{}: {} - increasing: {}", i - 1, set_of_nums[i - 1], is_increasing);
        if is_increasing && set_of_nums[i] <= set_of_nums[i - 1] {
            //  its suppose to be increasing, but we found a decrease
            is_safe = false;
            problems_counter += 1;
            if problems_counter == 1 {
              problem_index = i;
            }
            continue;
        }

        if !is_increasing && set_of_nums[i] >= set_of_nums[i - 1] {
            //  its suppose to be decreasing, but we found an increase
            is_safe = false;
            problems_counter += 1;
            if problems_counter == 1 {
              problem_index = i;
            }
            continue;
        }
        
        if (set_of_nums[i] - set_of_nums[i - 1]).abs() < 1 || (set_of_nums[i] - set_of_nums[i - 1]).abs() > 3 {
            // all numbers are decreasing by more than 1
            is_safe = false;
            problems_counter += 1;
            if problems_counter == 1 {
              problem_index = i;
            }
            continue;
        }
    }

    if is_safe {
        // println!("\tSafe");
        return true;
    } else {
      // its not safe, but if problems counter is 1, re-run it while removing that number
      if problems_counter == 1 {
        println!("\t\t{:?} problems_counter: {}", set_of_nums,  problems_counter);
        // let new_set_of_nums = set_of_nums.copy_within(0..problem_index, problem_index);
        let problem_num: i32 = set_of_nums.remove(problem_index);
        println!("\t\t{}", problem_num);
        println!("\t\t\tUpdated set of nums: {:?}", set_of_nums);

        
        let final_check: bool = check_for_safe_sequence(set_of_nums, problems_counter);
        println!("\t\t\t\tfinal_check: {}", final_check);

        if final_check {
          return true;
        } else {
          return false;
        }
      } else {
        println!("ERROR - {:?} problems_counter: {}", set_of_nums,  problems_counter);
        return false;
      }
    }
}