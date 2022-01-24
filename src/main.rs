use std::io;
use std::env;

mod helpers;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;

fn main() {
	
	let args: Vec<String> = env::args().collect();

	let day = match args.get(1) {
		Some(val) => {
			val.parse::<u32>()
		},
		_ => {
			println!("Pick a day:");
			let mut input = String::new();
			io::stdin()
				.read_line(&mut input)
				.expect("Failed to read line");
			input.trim().parse::<u32>()
		}
	};

	match day {
		Ok(1) => day1::run(),
		Ok(2) => day2::run(),
		Ok(3) => day3::run(),
		Ok(4) => day4::run(),
		Ok(5) => day5::run(),
		Ok(6) => day6::run(),
		Ok(7) => day7::run(),
		Ok(8) => day8::run(),
		Ok(9) => day9::run(),
		Ok(10) => day10::run(),
		Ok(11) => day11::run(),
		Ok(12) => day12::run(),
		Ok(13) => day13::run(),
		_ => println!("Not a valid day!"),
	};

	println!("DONE");
}
