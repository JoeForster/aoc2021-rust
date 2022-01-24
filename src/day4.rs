use std::io;

#[derive(Debug)]
struct BingoRow {
	numbers: Vec<u32>
}

#[derive(Debug)]
struct BingoBoard {
	rows: Vec::<BingoRow>,
	found_winner: bool
}

// TODO
/*
impl FromStr for BingoRow {

    type Err = ();

    fn from_str(input: &str) -> Result<Command, Self::Err> {

    }
}
*/

pub fn run() {


	let mut input = String::new();
	println!("Enter numbers:");

	io::stdin()
		.read_line(&mut input)
		.expect("Failed to read line");


	let numbers_to_draw: Vec<u32> = input
		.trim()
		.split(",")
        .into_iter()
        .map(|s| s.parse::<u32>().expect("Invalid u32 in drawn numbers"))
        .collect();

	println!("Numbers: {:?}", numbers_to_draw);

	let mut row_length = 0;
	let mut bingo_boards = Vec::<BingoBoard>::new();
	let mut next_board = BingoBoard { rows: Vec::<BingoRow>::new(), found_winner: false };
	let mut num_empty_lines = 0;
	
	loop {		
		let mut input = String::new();
		io::stdin()
			.read_line(&mut input)
			.expect("Failed to read line");

		let row_vals: Vec<&str> = input
			.trim()
			.split_whitespace()
			.collect();
			
		if row_vals.len() == 0 {
			
			if next_board.rows.len() != 0 {
				bingo_boards.push(next_board);
				next_board = BingoBoard { rows: Vec::<BingoRow>::new(), found_winner: false };
			}
			num_empty_lines = num_empty_lines + 1;
			if num_empty_lines > 1 {
				break;
			} else {
				continue;
			}
		} else {
			num_empty_lines = 0;
		}
			
		println!("row_vals: {:?}", row_vals);
			
		let row_vals: Vec<u32> = row_vals
			.into_iter()
			.map(|s| s.parse::<u32>().expect("Invalid u32 in row"))
			.collect();
			
		let num_vals : u32 = row_vals.len().try_into().unwrap();
		if num_vals != 0 {
			if row_length == 0 {
				row_length = num_vals;
			} else if num_vals != row_length {
				println!("Invalid row: {}", input);
				continue;
			}
		}
		assert!(num_vals == row_length);
		
		next_board.rows.push(BingoRow{numbers: row_vals});

	}

	for draw_up_to_index in 0..numbers_to_draw.len() {
		let drawn_numbers = &numbers_to_draw[0..draw_up_to_index];
		
		// TODO: Actually don't like making it mutable here just to track winner,
		// but otherwise we need a map or something to track winners
		for (board_index, board) in bingo_boards.iter_mut().enumerate() {
			let mut unmarked_sum = 0;
			//println!("board: {:?}", board);
			if board.found_winner {
				continue;
			}

			// TODO could this be made easier with some sort of matrix type?
			// 
			let mut winning_number_index = 0;
			for row in &board.rows {
				let mut winning_row = true;
				
				// Check each row for marked numbers, reporting if we find a winner.
				// As we go, we can sum the potential score of this board.
				for row_number in &row.numbers {
					let drawn_number_indexiter = drawn_numbers.iter().position(|&n| n == *row_number);
					match drawn_number_indexiter {
						Some(ix) => {
							if ix > winning_number_index {
								winning_number_index = ix;
							}
						},
						None => {
							winning_row = false;
							unmarked_sum = unmarked_sum + row_number;
						}
					}
				}
				// See if we won, but continue anyway to track score.
				if winning_row {
					board.found_winner = true;
				}
			}
			
			// We have a score sum now, but we may still need to find a column winner.
			// TODO: there's got to be a better way! 2D matrix?
			if !board.found_winner {
				for icol in 0..row_length-1 {
					let mut winning_col = true;
					for row in &board.rows {
						let check_against_num : u32 = row.numbers[icol as usize];
						let drawn_number_indexiter = drawn_numbers.iter().position(|&n| n == check_against_num);
						match drawn_number_indexiter {
							Some(ix) => {
								if ix > winning_number_index {
									winning_number_index = ix;
								}
							},
							None => {
								winning_col = false;
							}
						}
					}
					if winning_col {
						board.found_winner = true;
						break;
					}
				}
			}
			if board.found_winner {
				let winning_number = drawn_numbers[winning_number_index];
				println!("WIN on board #{} at drawn number index {}! Unmarked sum is: {} Winning number is: {} SCORE is: {}",
					board_index,
					draw_up_to_index,
					unmarked_sum,
					winning_number,
					unmarked_sum * winning_number);
			}
		}
	}	
}