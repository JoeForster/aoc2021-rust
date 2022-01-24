use std::io;
use std::str::FromStr;
use std::collections::HashMap;

#[derive(Debug)]
struct Digit {
	pattern: u8,
	pattern_num_bits: u8,
	output_number: Option<u8>
}

impl FromStr for Digit {

    type Err = ();

    fn from_str(input: &str) -> Result<Digit, Self::Err> {
		let mut pattern : u8 = 0;
		let mut num_bits_set = 0;
		for c in input.trim().chars() {
			pattern |= match c {
				'a' => (1 << 1),
				'b' => (1 << 2),
				'c' => (1 << 3),
				'd' => (1 << 4),
				'e' => (1 << 5),
				'f' => (1 << 6),
				'g' => (1 << 7),
				_ => return Err(())
			};
			num_bits_set += 1;
		}
		let output_number: Option<u8> = match num_bits_set {
			2 => Some(1),
			3 => Some(7),
			4 => Some(4),
			5 => None,
			6 => None,
			7 => Some(8),
			_ => return Err(()),
		};
		Ok( Digit{
				pattern: pattern,
				pattern_num_bits: num_bits_set,
				output_number: output_number} )
    }
}


#[derive(Debug)]
struct DigitPatterns {
	input_to_number: HashMap<u8, u8>,
	number_to_input: HashMap<u8, u8>
}


impl FromStr for DigitPatterns {

    type Err = ();

    fn from_str(input: &str) -> Result<DigitPatterns, Self::Err> {
		let input_patterns = input.trim().split_whitespace();
		let input_patterns = input_patterns.collect::<Vec<&str>>();
		if input_patterns.len() != 10 {
			return Err(());
		}
		
		let mut digit_patterns = DigitPatterns{
			input_to_number: HashMap::new(),
			number_to_input: HashMap::new() };
		let mut digits: Vec<Digit> = Vec::with_capacity(10);

		for digit_str in input_patterns {
			let digit: Digit = digit_str.parse().expect("Invalid digit in input patterns");
			match digit.output_number {
				Some(n) => {
						digit_patterns.number_to_input.insert(n, digit.pattern);
						digit_patterns.input_to_number.insert(digit.pattern, n);
					},
				None => ()
			};
			digits.push(digit);
		}
		assert!(digits.len() == 10, "Expected 10 digits in input patterns");
		
		let pattern_1: u8 = *digit_patterns.number_to_input.get(&1).expect("Did not identify 1 digit");
		let pattern_4: u8 = *digit_patterns.number_to_input.get(&4).expect("Did not identify 4 digit");
		assert!(digit_patterns.number_to_input.get(&7).is_some(), "Did not identify 7 digit");
		let pattern_8: u8 = *digit_patterns.number_to_input.get(&8).expect("Did not identify 8 digit");

		let d_b_bits = pattern_1 ^ pattern_4;		
		
		for digit in digits {
			// With 6 bits set, we have: 0, 6, 9
			if digit.pattern_num_bits == 6 {
				if digit.pattern | pattern_4 == pattern_8 {
					if digit.pattern | pattern_1 == digit.pattern {
						digit_patterns.number_to_input.insert(0, digit.pattern);
						digit_patterns.input_to_number.insert(digit.pattern, 0);
					} else {
						assert!(digit.pattern | pattern_1 == pattern_8);
						digit_patterns.number_to_input.insert(6, digit.pattern);
						digit_patterns.input_to_number.insert(digit.pattern, 6);
					}
				} else {
					digit_patterns.number_to_input.insert(9, digit.pattern);
					digit_patterns.input_to_number.insert(digit.pattern, 9);
				}
			// With 5 bits set, we have: 2, 3, 5
			} else if digit.pattern_num_bits == 5 {
				if digit.pattern | pattern_4 == pattern_8 {
					digit_patterns.number_to_input.insert(2, digit.pattern);
					digit_patterns.input_to_number.insert(digit.pattern, 2);
				} else if digit.pattern | pattern_1 == digit.pattern {
					digit_patterns.number_to_input.insert(3, digit.pattern);
					digit_patterns.input_to_number.insert(digit.pattern, 3);
				} else {
					assert!(digit.pattern | d_b_bits == digit.pattern);
					digit_patterns.number_to_input.insert(5, digit.pattern);
					digit_patterns.input_to_number.insert(digit.pattern, 5);
				}
			}
		}
		
		// We should know all the patterns now

		assert!(digit_patterns.number_to_input.get(&0).is_some(), "Did not identify 0 digit");
		assert!(digit_patterns.number_to_input.get(&2).is_some(), "Did not identify 2 digit");
		assert!(digit_patterns.number_to_input.get(&3).is_some(), "Did not identify 3 digit");
		assert!(digit_patterns.number_to_input.get(&5).is_some(), "Did not identify 5 digit");
		assert!(digit_patterns.number_to_input.get(&6).is_some(), "Did not identify 6 digit");
		assert!(digit_patterns.number_to_input.get(&9).is_some(), "Did not identify 9 digit");
		
		
		Ok( digit_patterns )
    }
}

#[derive(Debug)]
struct OutputValue {
	digits: Vec<Digit>//[Digit; 4]
}

fn read_digits(output_digits_str: &str) -> OutputValue {
	
	let output_digits: Vec<Digit> = output_digits_str.trim().split_whitespace().map(|s| s.parse().expect("Invalid digit pattern")).collect();
	assert!(output_digits.len() == 4, "Unexpected digit count");
	return OutputValue { digits: output_digits };
}

pub fn run() {


	let mut identified_count = 0;
	let mut outputs_sum = 0;
	
	loop {
		let mut input = String::new();
		println!("Enter the input:");
		io::stdin()
			.read_line(&mut input)
			.expect("Failed to read line");
		
		if input.trim().is_empty() {
			break;
		}

		let mut inputs_split = input
			.trim()
			.split("|");

		let patterns_str = inputs_split.next().expect("Not enough inputs in line");
		let patterns = patterns_str
			.parse::<DigitPatterns>()
			.expect("Invalid patterns");
			
		//println!("Digit patterns: {:?}", patterns);
		
		let digits_str = inputs_split.next().expect("Not enough inputs in line");
		let output = read_digits(digits_str);
		//println!("Digits from {}: {:?}", digits_str, output.digits);
		
		for digit in &output.digits {
			identified_count += match digit.output_number {
				Some(_) => 1,
				None => 0
			}				
		}
		
		let mut this_output_num: u64 = 0;
		for (pow_to_10, digit) in output.digits.iter().rev().enumerate() {
			let this_digit = patterns.input_to_number[&digit.pattern] as u64;
			let this_digit_val = 10_u64.pow(pow_to_10.try_into().unwrap()) * this_digit as u64;
			this_output_num += this_digit_val;
		}
		outputs_sum += this_output_num;
		println!("{} -> {}", patterns_str, this_output_num);
	}
	
	println!("Numbers derived from bit count (1, 4, 7, 8): {}", identified_count);
	println!("Total output sum: {}", outputs_sum);

}