use std::io;

fn parse_bits(input: &str) -> Result<u32, &str> 
{
	let input = input.trim();
	if input.len() == 0 {
		return Err(input)
	}
	
	let mut bitval = 1;
	let mut value : u32 = 0;
	for char in input.chars().rev() {
		value = match char {
			'0' => value,
			'1' => value + bitval,
			_ 	=> return Err(input)
		};
		bitval = bitval * 2;
	}
	Ok(value)
}

fn day3_part1(v: &Vec<u32>, num_bits: u32) {
	
	let mut gamma_rate = 0;
	let mut epsilon_rate = 0;
	let mut bit = 0;
	
	while bit < num_bits {
		let bit_mask = 2_u32.pow(bit);
		let mut count_0 = 0;
		let mut count_1 = 0;
		for n in v {
			match n & bit_mask {
				0 => count_0 = count_0 + 1,
				_ => count_1 = count_1 + 1,
			}
		}
		if count_1 >= count_0 {
			gamma_rate = gamma_rate + bit_mask;
		} else {
			epsilon_rate = epsilon_rate + bit_mask;
		}
		bit = bit + 1;
	}
	
	let power = u64::from(gamma_rate) * u64::from(epsilon_rate);
	println!("gamma_rate: {} epsilon_rate: {} power: {}", gamma_rate, epsilon_rate, power);
}

fn day3_part2(v: &Vec<u32>, num_bits: u32) {

	let mut oxygen_values = v.to_vec();
	{
		let mut bit = num_bits - 1;

		loop {

			let bit_mask = 2_u32.pow(bit);
			let mut count_0 = 0;
			let mut count_1 = 0;
			for n in &oxygen_values {
				match n & bit_mask {
					0 => count_0 = count_0 + 1,
					_ => count_1 = count_1 + 1,
				}
			}
			
			if count_1 >= count_0 {
				oxygen_values.retain(|&n| n & bit_mask != 0);
			} else {
				oxygen_values.retain(|&n| n & bit_mask == 0);
			}
		
			assert!(oxygen_values.len() > 0);
			if oxygen_values.len() == 1 || bit == 0 {
				break;
			}
			bit = bit - 1;
		}
	}
	
	let mut co2_values = v.to_vec();
	{
		let mut bit = num_bits - 1;

		loop {
			
			let bit_mask = 2_u32.pow(bit);

			let co2_values_str: String = co2_values.iter().map( |&n| format!("{:b},", n) ).collect();
			println!("co2_values: {} num_bits: {} bit_mask: {:b}", co2_values_str, num_bits, bit_mask);

			let mut count_0 = 0;
			let mut count_1 = 0;
			for n in &co2_values {
				match n & bit_mask {
					0 => count_0 = count_0 + 1,
					_ => count_1 = count_1 + 1,
				}
			}
			
			if count_1 >= count_0 {
				co2_values.retain(|&n| n & bit_mask == 0);
			} else {
				co2_values.retain(|&n| n & bit_mask != 0);
			}

			assert!(co2_values.len() > 0);
			if co2_values.len() == 1 || bit == 0 {
				break;
			}
			bit = bit - 1;
		}
	}

	let oxygen_value = oxygen_values[0];
	let co2_value = co2_values[0];
	println!("oxygen_value: {} (out of {} values) co2_values: {} (out of {} values)",
		oxygen_value, oxygen_values.len(), co2_value, co2_values.len());

	let life_support = oxygen_value * co2_value;
	println!("life_support: {}", life_support);
	
}

pub fn run() {

	let mut num_bits : u32 = 0;
	let mut v: Vec<u32> = Vec::new();
	loop {
		let mut input = String::new();
		println!("Type something:");

		io::stdin()
			.read_line(&mut input)
			.expect("Failed to read line");

		println!("Here is the thing you typed: {}", input);
		
		// Annoying: variable number of bits in input depends on strings passed so can't calculate numerically here
		let input = input.trim();

		let value = match parse_bits(&input) {
            Ok(value) => value,
            Err(_) => break,
        };
		
		let this_num_bits : u32 = input.len().try_into().unwrap();
		
		if num_bits == 0 {
			num_bits = this_num_bits;
		} else if num_bits != this_num_bits {
			break;
		}

		v.push(value);

	}
	println!("Here are the values you typed:");
	for n in &v {
		println!("'{}'", n);
	}
	
	day3_part1(&v, num_bits);
	day3_part2(&v, num_bits);
}