use std::io;
use std::fmt;
//use std::str::FromStr;
//
//#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
//struct Point {
//	x: usize,
//	y: usize
//}

const RADIX: u32 = 10;

fn read_line() -> Option<String> {
	let mut input = String::new();
	println!("Enter the input:");
	io::stdin()
		.read_line(&mut input)
		.expect("Failed to read line");
	
	match input.trim() {
		"" => None,
		input_trimmed => Some(input_trimmed.to_string())
	}
}

#[derive(Debug)]
struct GridItem {
	val: i32,
	flashed : bool,
}

#[derive(Debug)]
struct NumberGrid {
	vals: Vec<Vec<GridItem>>,
	width: usize
}

impl fmt::Display for NumberGrid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		for row in &self.vals {
			for item in row {
				let display_val = if item.val > 9 { 0 } else { item.val };
				write!(f, "{}", display_val).expect("Bad write");
			}
			write!(f, "\n").expect("Bad write");
		}
		Ok(())
    }
}


fn read_number_grid() -> NumberGrid {

	let mut vals: Vec<Vec<GridItem>> = Vec::new();	
	let mut width: usize = 0;
	loop {
		let input = read_line();
		match input {
			None => break,
			Some(input) => {
				let row_vals: Vec<GridItem> = input
					.trim()
					.chars()
					.map(|c| GridItem{ val: c.to_digit(RADIX).expect("Invalid digit") as i32, flashed: false})
					.collect();
				let this_len: usize = row_vals.len();
				assert!(this_len > 0, "Non-empty but invalid row");
				match width {
					0 => width = row_vals.len(),
					_ => assert!(width == this_len, "Inconsistent row widths")
				}

				vals.push(row_vals);
			}
		}
	}
	NumberGrid{ vals, width }
}

fn check_flash(grid: &mut NumberGrid, x: usize, y: usize) -> bool {
	
	let height = grid.vals.len();
	if !grid.vals[y][x].flashed && grid.vals[y][x].val > 9 {
		
		// FLASH
		grid.vals[y][x].flashed = true;
		
		// Increment energy level of each adjacent
		if x > 0 {
			grid.vals[y][x-1].val += 1; // left
			if y > 0 {
				grid.vals[y-1][x-1].val += 1; // up+left
			}
			if y < height-1 {
				grid.vals[y+1][x-1].val += 1; // down+left
			}
		}
		
		if x < grid.width-1 {
			grid.vals[y][x+1].val += 1; // right
			if y > 0 {
				grid.vals[y-1][x+1].val += 1; // up+right
			}
			if y < height-1 {
				grid.vals[y+1][x+1].val += 1; // down+right
			}
		}
		
		if y > 0 {
			grid.vals[y-1][x].val += 1; // up
		}
		if y < height-1 {
			grid.vals[y+1][x].val += 1; // down
		}
		
		//println!("FLASH at {}, {}", x, y);
		//println!("{}", grid);
		return true;
	} else {
		return false;
	}
}

fn run_step(grid: &mut NumberGrid) -> usize {
	// First pass: just increase energy levels
	for row in &mut grid.vals {
		for val in row {
			val.val += 1;
		}
	}

	// Repeat pass to check for flashes until we see no more flashes
	let mut num_flashes = 0;
	{
		assert!(grid.width > 0);
		let height = grid.vals.len();
		let mut remaining_checks = 2; // Need to check one extra time in case of chain reactions
		while remaining_checks > 0 {
			for y in 0..height {
				for x in 0..grid.width {
					//println!("check_flash {} {}", x, y);
					let flashed = check_flash(grid, x, y);
					if flashed {
						num_flashes += 1;
						remaining_checks = 2;
					}
				}
			}
			remaining_checks -= 1;
		}
	}

	// Reset all "flashed" flags and energy to 0
	for row in &mut grid.vals {
		for val in row {
			//println!("CHECK {} FLASHED {}", val.val, val.flashed);
			if val.flashed {
				assert!(val.val > 9);
				val.flashed = false;
				val.val = 0;
			} else {
				assert!(val.val <= 9);
				
			}
		}
	}

	return num_flashes;
}

pub fn run() {

	let mut number_grid = read_number_grid();
	println!("number_grid: {:?}", number_grid);
	
	let total_steps = 500;
	let mut total_flashes: usize = 0;
	for step in 1..=total_steps {
		let num_flashes = run_step(&mut number_grid);
		println!("Flashes after step {}: {}", step, num_flashes);
		println!("{}", number_grid);
		total_flashes += num_flashes;
		
		if num_flashes == number_grid.width * number_grid.vals.len() {
			println!("SYNCHRONISED after {} steps!", step);
			break;
		}
	}
	println!("Total flashes: {}", total_flashes);
}
