use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Coord {
	row: usize,
	col: usize
}

fn main() {
	println!("Hello, world!");
	// Data file must be same sized row and columns (in our case 140x140, or in the example 10x10)
	let f: File = std::fs::File::open("easy.txt").expect("Could not open input file");
	let reader: BufReader<File> = BufReader::new(f);
	let mut line_counter: i32 = 0;
	// 1. Insert every character from every line in the data file into a 2D Matrix
	let mut graph: Vec<Vec<char>> = Vec::new();
	let mut solutions: Vec<(Coord, Coord)> = Vec::new();
	let pattern: &str = "XMAS";

	for line in reader.lines() {
		let line: String = line.unwrap();
		println!("{}: Line: {} - {}", line_counter, line, line.len());
		let mut row: Vec<char> = Vec::new();
		for c in line.chars() {
			row.push(c);
		}
		graph.push(row);

		line_counter += 1;
	}

	println!("\n\nGraph: {:?}\n\n", graph);

	let example_possible_routes: Vec<Vec<Coord>> = get_possible_adjacent_routes(&graph, pattern, Coord { row: 2, col: 3 });
	println!("Possible Routes: {:?}", example_possible_routes);

	println!("**********************************************");
	// 2. Search for the pattern 'XMAS' in the 2D Matrix
	for row in 0..graph.len() {
		for col in 0..graph[row].len() {
			println!("Row: {} | Col: {} | Char: {}", row, col, graph[row][col]);
			// Start position is row, col
			let start_coord: Coord = Coord { row, col };

			// Check the surrounding characters (top, bottom, left, right, diaganols in all directions as well) of the current position to see if it matches the next character in the pattern
			// let possible_routes: Vec<Vec<Coord>> = get_possible_adjacent_routes(&graph, pattern, start_coord);
			// println!("\tPossible Routes: {:?}", possible_routes.len());
			// match check_surroundings(pattern, &graph, start_coord, start_coord, 0) {
			// 	Some(end_coord) => {
			// 		println!("Pattern Found - Start Position: {:?} | End Position: {:?}", start_coord, end_coord);
			// 		// Check if the pattern is already found i.e., start and end positions already exist in the set of solutions
			// 		for solution in solutions.clone() {
			// 			// check if the start and end positions are the same or the reverse
			// 			if (solution.0 == start_coord && solution.1 == end_coord) || (solution.0 == end_coord && solution.1 == start_coord) {
			// 				println!("Pattern Already Found - Start Position: {:?} | End Position: {:?}", start_coord, end_coord);
			// 				return;
			// 			} else {
			// 				// Add it to the set of solutions
			// 				solutions.push((start_coord, end_coord));
			// 			}
			// 		}
			// 	},
			// 	None => println!("Pattern Not Found")
			// }
		}
	}

	println!("**********************************************");
	println!("Solutions: {:?}", solutions.len());
	
	
	
	// Go through the graph and check if it matches the start of the end of the pattern 'XMAS' via Depth First Search
	// If it does, then we can start to check the surrounding characters to see if it matches the pattern
		// If the entire pattern is found
			// check if already found
				// not found => then we add it to the list of patterns found
				// found => ignore
}

// fn check_surroundings(pattern: &str, graph: &Vec<Vec<char>>, start_position: Coord, current_position: Coord, pattern_index: usize) -> Option<Coord> {
// 	// Check if the pattern_index is the end of the pattern - DFS
// 	// Check if the current position's value in the graph matches the pattern's current index
// 	let char_value: char = graph[current_position.row][current_position.col];
	
// 	println!("Start Position: {:?} - Current Position: {:?} | Current Value: {}, Pattern Index: {}", start_position, current_position, char_value, pattern_index);

// 	match pattern.chars().nth(pattern_index as usize) {
// 		Some(current_char) => {
// 			if char_value == current_char {
// 				println!("\t\tPattern Matched: {}", current_char);
// 				if pattern_index == pattern.len() {
// 					return Some(current_position); // Return the end position
// 				}
// 				println!("\t\tAdjacent Positions: {:?}", adj_pos);
// 				for next_coord in adj_pos {
// 					match check_surroundings(pattern, graph, start_position, next_coord, pattern_index + 1) {
// 						Some(next_position) => return Some(next_position),
// 						None => continue
// 					}
// 				}
// 				// If it does, then recursively call check_surroundings with the new current position and pattern index
// 				// If it doesn't, then return None
// 				return None;
// 			} else {
// 				// Doesn't match the pattern's value at the current index, so return None
// 				return None;
// 			}
// 		},
// 		None => {
// 			// Pattern index is out of bounds, so return None
// 			println!("Pattern Index Out of Bounds");
// 			return None;
// 		}
// 	}

// }

// fn found_pattern(solutions: mut Vec<(Coord, Coord)>, start_position: Coord, end_position: Coord) {
// 	// Check if the pattern is already found i.e., start and end positions already exist in the set of solutions
// 	for solution in solutions {
// 		// check if the start and end positions are the same or the reverse
// 		if (solution.0 == start_position && solution.1 == end_position) || (solution.0 == end_position && solution.1 == start_position) {
// 			println!("Pattern Already Found - Start Position: {:?} | End Position: {:?}", start_position, end_position);
// 			return;
// 		} else {
// 			// Add it to the set of solutions
// 			solutions.push((start_position, end_position));
// 		}
// 	}
// }

fn get_possible_adjacent_routes(graph: &Vec<Vec<char>>, pattern: &str, current_position: Coord) -> Vec<Vec<Coord>> {
	// Graph is a 2D matrix, list of rows
	// Get every adjacent position to the current position
	let mut adjacent_possibilities: Vec<Vec<Coord>> = Vec::new();
	// Check if the adjacent positions are within the bounds of the graph i.e., 0 <= row < graph.len() and 0 <= col < graph[row].len()
	println!("Current Position: {:?} - {}", (current_position.row, current_position.col), graph[current_position.col][current_position.row]);
	let x_start: usize = 0;
	let y_start: usize = 0;
	let max_x_scale: usize = graph[current_position.col].len() - 1;
	let max_y_scale: usize = graph.len() - 1;
	let pattern_length: usize = pattern.len() - 1;
	println!("Row Scale: {} | Col Scale: {} | Pattern Length: {}", max_x_scale, max_y_scale, pattern_length);
	
	// We are getting the adjacent positions in a clockwise direction starting from the top
	// We need to go at a depth of the pattern length for all directions if the coordinates are within the bounds of the graph
	// If the coordinates are out of bounds, then we ignore them
	// so if x is less than the length of the pattern i.e. (XMAS - 4), then we can't have a top coordinate
	// do the same for the rest of the directions...top left, top right, bottom, bottom left, bottom right, left, right
	
	// Check if x coordinate is greater than 3 to determine if we have a possible route starting from the position directly on top
	if current_position.row >= (x_start + pattern_length) {
		let mut top_route: Vec<Coord> = Vec::new();
		for i in 0..pattern_length {
			let top_coord: Coord = Coord { row: current_position.row - i, col: current_position.col };
			top_route.push(top_coord);
		}
		adjacent_possibilities.push(top_route);
	}

	if current_position.row <= (max_x_scale - pattern_length) {
		let mut bottom_route: Vec<Coord> = Vec::new();
		for i in 0..pattern_length {
			let bottom_coord: Coord = Coord { row: current_position.row + i, col: current_position.col };
			bottom_route.push(bottom_coord);
		}
		adjacent_possibilities.push(bottom_route);
	}

	if current_position.col >= max_y_scale - pattern_length {
		let mut left_route: Vec<Coord> = Vec::new();
		for i in 1..pattern_length {
			let left_coord: Coord = Coord { row: current_position.row, col: current_position.col - i };
			left_route.push(left_coord);
		}
		adjacent_possibilities.push(left_route);
	}

	if current_position.col <= max_y_scale - pattern_length {
		let mut right_route: Vec<Coord> = Vec::new();
		for i in 1..pattern_length {
			let right_coord: Coord = Coord { row: current_position.row, col: current_position.col + i };
			right_route.push(right_coord);
		}
		adjacent_possibilities.push(right_route);
	}

	// Check if there is a top left diagonal
	if current_position.row >= pattern_length && current_position.col >= pattern_length {
		let mut top_left_diagonal_route: Vec<Coord> = Vec::new();
		for i in 0..pattern_length {
			let top_left_diagonal_coord: Coord = Coord { row: current_position.row - i, col: current_position.col - i };
			top_left_diagonal_route.push(top_left_diagonal_coord);
		}
		adjacent_possibilities.push(top_left_diagonal_route);
	}

	// Check if there is a top right diagonal
	if current_position.row <= max_x_scale - pattern_length && current_position.col <= max_y_scale - pattern_length {
		let mut top_right_diagonal_route: Vec<Coord> = Vec::new();
		for i in 0..pattern_length {
			let top_right_diagonal_coord: Coord = Coord { row: current_position.row + i, col: current_position.col + i };
			top_right_diagonal_route.push(top_right_diagonal_coord);
		}
		adjacent_possibilities.push(top_right_diagonal_route);
	}

	// Check if there is a bottom left diagonal
	if current_position.row >= pattern_length && current_position.col <= max_y_scale - pattern_length {
		let mut bottom_left_diagonal_route: Vec<Coord> = Vec::new();
		for i in 0..pattern_length {
			let bottom_left_diagonal_coord: Coord = Coord { row: current_position.row - i, col: current_position.col + i };
			bottom_left_diagonal_route.push(bottom_left_diagonal_coord);
		}
		adjacent_possibilities.push(bottom_left_diagonal_route);
	}

	// Check if there is a bottom right diagonal
	if current_position.row <= max_x_scale - pattern_length && current_position.col >= pattern_length {
		let mut bottom_right_diagonal_route: Vec<Coord> = Vec::new();
		for i in 0..pattern_length {
			let bottom_right_diagonal_coord: Coord = Coord { row: current_position.row + i, col: current_position.col - i };
			bottom_right_diagonal_route.push(bottom_right_diagonal_coord);
		}
		adjacent_possibilities.push(bottom_right_diagonal_route);
	}

	adjacent_possibilities
}

// 
// 	if current_position.row > 0 {
// 		// if x is greater than 0 then we know there is a row above it, thus we can have a top coordinate
// 		let top_coord: Coord = Coord { row: current_position.row - 1, col: current_position.col };
// 		adjacent_positions.push(top_coord); // Top

// 		// Now can check if there is a top left diagonal
// 		if current_position.col > 0 {
// 			let top_left_coord: Coord = Coord { row: current_position.row - 1, col: current_position.col - 1 };
// 			adjacent_positions.push(top_left_coord); // Top Left Diagonal
// 		}

// 		// Now can check if there is a top right diagonal
// 		if current_position.col < max_y_scale {
// 			let top_right_coord: Coord = Coord { row: current_position.row - 1, col: current_position.col + 1};
// 			adjacent_positions.push(top_right_coord); // Top Right Diagonal
// 		}
// }

// if current_position.row < max_x_scale {
// 	let bottom_cord: Coord = Coord { row: current_position.row + 1, col: current_position.col };
// 	adjacent_positions.push(bottom_cord); // Bottom

// 	// Now can check if there is a bottom left diagonal
// 	if current_position.col > 0 {
// 		let bottom_left_coord: Coord = Coord { row: current_position.row + 1, col: current_position.col - 1 };
// 		adjacent_positions.push(bottom_left_coord); // Bottom Left Diagonal
// 	}

// 	// Now can check if there is a bottom right diagonal
// 	if current_position.col < max_y_scale {
// 		let bottom_right_coord: Coord = Coord { row: current_position.row + 1, col: current_position.col + 1 };
// 		adjacent_positions.push(bottom_right_coord); // Bottom Right Diagonal
// 	}
// }

// if current_position.col > 0 {
// 	let left_coord: Coord = Coord { row: current_position.row, col: current_position.col - 1 };
// 	adjacent_positions.push(left_coord); // Left
// }

// if current_position.col < max_y_scale {
// 	let right_coord: Coord = Coord { row: current_position.row, col: current_position.col + 1 };
// 	adjacent_positions.push(right_coord); // Right
// }
