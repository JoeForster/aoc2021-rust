use std::io;

fn is_opener_char(c: char) -> bool {
	match c {
		'(' => true,
		'[' => true,
		'{' => true,
		'<' => true,
		_ => false
	}
}

fn is_closer_char(c: char) -> bool {
	match c {
		')' => true,
		']' => true,
		'}' => true,
		'>' => true,
		_ => false
	}
}

fn get_close_char(c: char) -> Option<char> {
	match c {
		'(' => Some(')'),
		'[' => Some(']'),
		'{' => Some('}'),
		'<' => Some('>'),
		_ => None
	}
}

fn get_error_score(c: char) -> u64 {
	match c {
		'(' => 3,
		')' => 3,
		'[' => 57,
		']' => 57,
		'{' => 1197,
		'}' => 1197,
		'<' => 25137,
		'>' => 25137,
		_ => 0
	}
}

fn get_complete_score(c: char) -> u64 {
	match c {
		'(' => 1,
		')' => 1,
		'[' => 2,
		']' => 2,
		'{' => 3,
		'}' => 3,
		'<' => 4,
		'>' => 4,
		_ => 0
	}
}

pub fn run() {

	let mut file_corrupted = false;
	let mut error_score: u64 = 0;
	let mut complete_scores: Vec<u64> = Vec::new();

	loop {
		
		let mut input = String::new();
		println!("Enter the input:");
		io::stdin()
			.read_line(&mut input)
			.expect("Failed to read line");
		
		if input.trim().is_empty() {
			break;
		}
		println!("FOR LINE {}", input.trim());
		
		let mut line_corrupted = false;
		let mut stack: Vec<char> = Vec::new();
		for (i, c) in input.trim().chars().enumerate() {
			if is_opener_char(c) {
				stack.push(c);
			} else if is_closer_char(c) {
				match stack.pop() {
					None => {
						println!("    -> is CORRUPTED due to unmatched closer char {} at index {}", c, i); 
						line_corrupted = true;
					},
					Some(opener_char) => {
						let expected_closer = get_close_char(opener_char).unwrap();
						if c != expected_closer {
							println!("    -> is CORRUPTED due to unexpected closer char {} at index {} (expected {})", c, i, expected_closer);
							line_corrupted = true;
							error_score += get_error_score(c);
						}
					}
				}
			}
			else {
				println!("    -> is CORRUPTED due to invalid char {} at index {}", c, i);
				line_corrupted = true;
			}
		}
		
		if line_corrupted {
			file_corrupted = true;
		} else {
			match stack.len() {
				0 => println!("is VALID"),
				remaining_len => {
					println!("    -> is INCOMPLETE, remaining chars: {}", remaining_len);
					let mut score: u64 = 0;
					for c in stack.iter().rev() {
						score *= 5;
						score += get_complete_score(*c);
					}
					complete_scores.push(score);
					println!("    -> completed line would be: {}{} - score {}",
						input.trim(),
						stack.iter().rev().map(|c| get_close_char(*c).unwrap()).collect::<String>(),
						score);
				}
			}
		}
		
	}
	
	if file_corrupted {
		println!("CORRUPT file ERROR SCORE = {}", error_score);
	}
	
	assert!(complete_scores.len() % 2 != 0, "Expected odd number of scores");
	complete_scores.sort();
	println!("Auto-complete scores: {:?}", complete_scores);
	let middle_index = complete_scores.len() / 2;
	println!("Middle score: {}", complete_scores[middle_index]);
	
}