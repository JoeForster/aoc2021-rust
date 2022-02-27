use crate::helpers::FileReader;
use crate::helpers::read_answer;

fn read_input() -> Vec<u32> {
	let mut input_reader = FileReader::open("inputs/day1.txt").expect("Failed to open file");

	let mut v: Vec<u32> = Vec::new();
	let mut buffer = String::new();
	while let Some(_line) = input_reader.read_line(&mut buffer) {
		let number: u32 = match buffer.trim().parse() {
			Ok(number) => number,
			Err(_) => break,
		};

		//println!("Here is the number you typed: {}", number);
		v.push(number);
	}
	println!("Here's the input:");
	for n in &v {
		println!("{}", n);
	}
	return v;
}

fn part1(v: &Vec<u32>) -> String {
	
	let mut increases = 0;
	let mut prev : u32 = 0;
	for n in v {
		if &prev > &0 && n > &prev
		{
			increases = increases + 1;
		}
		prev = *n
	}
	println!("Number of increases: {}", increases);
	return increases.to_string();

}

fn part2(v: &Vec<u32>) -> String {

	let mut increases = 0;
	let mut prev : u32 = 0;
	let mut index = 0;
	while index+2 < v.len()
	{
		let window_sum = v[index] + v[index+1] + v[index+2];
		if &prev > &0 && window_sum > prev
		{
			increases = increases + 1;
		}
		prev = window_sum;
		index = index + 1;
	}
	println!("Number of increases with sliding windows: {}", increases);
	return increases.to_string();
}

pub fn run() {
	
	let v = read_input();
	
	// TODO: generalise to all the other days for unit testing without all this repetition.
	let part1_answer = read_answer(1, 1);
	let part1_result = part1(&v);
	assert!(part1_result == part1_answer);

	let part2_answer = read_answer(1, 2);
	let part2_result = part2(&v);
	assert!(part2_result == part2_answer);
}
