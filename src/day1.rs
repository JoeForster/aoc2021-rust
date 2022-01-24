use std::io;

fn day1_part1(v: &Vec<u32>) {
	
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

}

fn day1_part2(v: &Vec<u32>) {

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

}

pub fn run() {
	let mut v: Vec<u32> = Vec::new();
	loop {			
		let mut input = String::new();
		println!("Type something:");

		io::stdin()
			.read_line(&mut input)
			.expect("Failed to read line");

		//println!("Here is the thing you typed: {}", input);

		let number: u32 = match input.trim().parse() {
			Ok(number) => number,
			Err(_) => break,
		};

		//println!("Here is the number you typed: {}", number);
		v.push(number);

	}
	println!("Here are the numbers you typed:");
	for n in &v {
		println!("{}", n);
	}
	
	day1_part1(&v);

	day1_part2(&v);
}