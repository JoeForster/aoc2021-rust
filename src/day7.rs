use std::io;


fn calc_fuel_cost(x_positions: &Vec<i32>, x_target: i32) -> i32 {
	let cost_to_get_to_x = |x: &i32| (x-x_target).abs();
	x_positions.into_iter().map(cost_to_get_to_x).sum()
}

// This looks cool, but could probably be faster?
fn calc_fuel_cost_increasing(x_positions: &Vec<i32>, x_target: i32) -> i32 {
	let cost_to_get_to_x = |x: &i32| (0..=(x-x_target).abs()).sum::<i32>();
	x_positions.into_iter().map(cost_to_get_to_x).sum()
}

pub fn run() {

	let mut input = String::new();
	println!("Enter the input:");
	io::stdin()
		.read_line(&mut input)
		.expect("Failed to read line");
		
	
	let x_positions_str = input
		.trim()
		.split(",")
		.collect::<Vec<&str>>();
		
	if x_positions_str.is_empty() {
		panic!("Empty input");
	}		
	
		
	let mut min_x = i32::MAX;
	let mut max_x = i32::MIN;
	let mut x_positions = Vec::<i32>::with_capacity(x_positions_str.len());
	for x_str in x_positions_str {
		let x : i32 = x_str.parse().expect("Invalid input i32");
		min_x = x.min(min_x);
		max_x = x.max(max_x);
		x_positions.push(x);
	}
	assert!(!x_positions.is_empty());

	{
		let mut lowest_cost = i32::MAX;
		let mut lowest_cost_target_x = 0;
		for target_x in min_x..=max_x {
			let this_cost = calc_fuel_cost(&x_positions, target_x);
			println!("Fuel cost for target pos {}: {}", target_x, this_cost);
			if this_cost < lowest_cost {
				lowest_cost = this_cost;
				lowest_cost_target_x = target_x;
			}
		}
		println!("Cheapest target x is {} with fuel cost {}", lowest_cost_target_x, lowest_cost);
	}

	{
		let mut lowest_cost = i32::MAX;
		let mut lowest_cost_target_x = 0;
		for target_x in min_x..=max_x {
			let this_cost = calc_fuel_cost_increasing(&x_positions, target_x);
			println!("CORRECTED fuel cost for target pos {}: {}", target_x, this_cost);
			if this_cost < lowest_cost {
				lowest_cost = this_cost;
				lowest_cost_target_x = target_x;
			}
		}
		println!("Cheapest target x is {} with CORRECTED fuel cost {}", lowest_cost_target_x, lowest_cost);
	}
}