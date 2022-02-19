use std::fmt;
//use std::str::FromStr;
//use std::collections::HashMap;
//use std::collections::HashSet;

// TODO to impl display
//struct Row {
//	vals: Vec<bool>
//}

//use crate::helpers::helpers::read_line;
use crate::helpers::FileReader;

#[derive(Debug, Clone, PartialEq)]
enum FoldBy {
	X(usize),
	Y(usize)
}

#[derive(Debug, Clone)]
struct Map {
	nodes: Vec< Vec<bool> >,
	fold: Vec< FoldBy >
}


impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		//write!(f, "    ").expect("Bad write");
		//for x in 0..self.nodes[0].len() {
		//	write!(f, "{:0>2} ", x).expect("Bad write");
		//
		//write!(f, "\n").expect("Bad write");
		let next_fold = self.fold.iter().next();
		for (y, row) in self.nodes.iter().enumerate() {
			//write!(f, "{:0>2} ", y).expect("Bad write");
			for (x, node) in row.iter().enumerate() {
				// TODO: there HAS to be a way to do this without a mutable!!
				let mut display_val = if *node { "#" } else { "." };				
				if !*node && next_fold.is_some() {
					if next_fold.unwrap() == &FoldBy::X(x) {
						display_val = "|";
					} else if next_fold.unwrap() == &FoldBy::Y(y) {
						display_val = "-";
					}
				}
				write!(f, "{}", display_val).expect("Bad write");
			}
			write!(f, "\n").expect("Bad write");
		}
		//for fold_by in &self.fold {
		//	writeln!(f, "Fold by {:?}", fold_by).expect("Bad write");
		//}
		Ok(())
    }
}


fn read_grid() -> Map {
	let mut nodes : Vec< Vec<bool> > = Vec::new();
	let mut fold : Vec< FoldBy > = Vec::new();
	
	let mut width : usize = 0; 
	let mut height : usize = 0; 
	let mut buffer = String::new();
	
	let mut file_reader = FileReader::open("day13_input.txt").expect("Failed to open file");
	//let mut file_reader = FileReader::open("day13_input_example.txt").expect("Failed to open file");
	
	while let Some(_line) = file_reader.read_line(&mut buffer) {
		// Note the "empty line" will still have an endline char.
		//println!("Buffer: '{}'", buffer);
		if buffer.trim().is_empty() {
			break;
		}
		
		let mut two_numbers = buffer.trim().split(",");
		let x : usize = two_numbers.next()
			.expect("Bad coordinate format")
			.parse()
			.expect("bad number format");
		let y : usize = two_numbers.next()
			.expect("Bad coordinate format")
			.parse()
			.expect("bad number format");

		// Grow the grid to fit the input coordinate, as needed
		while nodes.len() <= y {
			nodes.push( Vec::new() );
		}				
		while nodes[y].len() <= x {
			nodes[y].push(false);
		}
		nodes[y][x] = true;
		
		let new_width = x+1;
		if new_width > width {
			width = new_width;
		}
		let new_height = y+1;
		if new_height > height {
			height = y+1;
		}
	}
	
	// Normalise (fill so width is consistent)
	// AND for folding to work the width and height must be odd.#
	// NOTE I realise now this is unnecessary,
	// since it is now fixed in the fold read below, since we need to be able to read 1 out of bounds after folding too
	//if width % 2 == 0 {
	//	width += 1;
	//}
	for row in nodes.iter_mut() {
		while row.len() < width {
			row.push(false);
		}
	}	
	//if height % 2 == 0 {
	//	nodes.push(vec![false; width]);
	//}

	while let Some(_line) = file_reader.read_line(&mut buffer) {
		if let Some(fold_x_str) = buffer.trim().strip_prefix("fold along x=") {
			let fold_x = fold_x_str.parse().expect("Bad fold coord number");
			fold.push( FoldBy::X(fold_x) ) ;
		} else if let Some(fold_y_str) = buffer.trim().strip_prefix("fold along y=") {
			let fold_y = fold_y_str.parse().expect("Bad fold coord number");
			fold.push( FoldBy::Y(fold_y) );
		}
	}


	Map{ nodes, fold }
}

fn process_folds(map : &Map) -> Map {
	let mut folded_maps : Vec<Map> = vec![ map.clone() ];
	
	for fold_instruction in &map.fold {
		let prev_map = folded_maps.last().unwrap();		
		println!("FOLDING MAP by {:?}", fold_instruction);
		println!("{}", prev_map);
		
		
		let folded_map_rows = match fold_instruction {
			FoldBy::X(fold_by_x) => {
				let num_rows = prev_map.nodes.len();
				let num_columns = prev_map.nodes[0].len();
				let last_column_index = num_columns-1;
				let num_columns_folded = last_column_index/2;
				assert!(*fold_by_x == num_columns_folded);
				assert!(num_columns % 2 == 1); // For a valid fold we need an odd number
				let mut folded_map_rows : Vec< Vec<bool> > = prev_map.nodes.clone();
				for row_index in 0..num_rows {
					folded_map_rows[row_index].truncate(num_columns_folded);
					assert!(num_columns == prev_map.nodes[row_index].len());
					//println!("fold row {} -> new width {}", row_index, num_columns_folded);
					for x in 0..*fold_by_x-1 {
						let fold_against = last_column_index - x;
						
						//println!("[{}][{}] <- [{}][{}]={} || [{}][{}]={}",
						//	row_index, x,
						//	row_index, x, prev_map.nodes[row_index][x],
						//	row_index, fold_against, prev_map.nodes[row_index][fold_against]);
						folded_map_rows[row_index][x] = prev_map.nodes[row_index][x] || prev_map.nodes[row_index][fold_against];
					}
					//println!("row {} w={} result: {:?}", row_index, num_columns_folded, folded_map_rows[row_index]);
				}
				folded_map_rows
			},
			FoldBy::Y(fold_by_y) => {
				// Note that we must deal with uneven folds here - folding upwards,
				// so some rows at the top may be left alone.
				let num_rows = prev_map.nodes.len();
				let num_columns = prev_map.nodes[0].len();
				let num_rows_folded = *fold_by_y;
				assert!(*fold_by_y < num_rows);
				//assert!(num_rows % 2 == 1); // For a valid fold we need an odd number
				let mut folded_map_rows : Vec< Vec<bool> > = prev_map.nodes[0..*fold_by_y].to_vec();
				println!("fold_by_y: {}", fold_by_y);
				println!("folded_map_rows.len(): {}", folded_map_rows.len());
				println!("num_rows_folded: {}", num_rows_folded);
				
				// Verify: fold is at middle (or requires 1 read over)
				assert!(folded_map_rows.len() == num_rows_folded);

				// VERIFY: "dots will never appear exactly on a fold line."
				assert!(prev_map.nodes[*fold_by_y].len() == num_columns);
				for x in 0..num_columns {
					assert!(prev_map.nodes[*fold_by_y][x] == false);
				}	
						
				for row_index in *fold_by_y+1..num_rows {
					let dist = row_index - fold_by_y;
					assert!(dist > 0);					
					let fold_against = fold_by_y - dist;
					let source_row = &prev_map.nodes[row_index];
					let target_row = &mut folded_map_rows[fold_against];
					let width = target_row.len();
					println!("fold row {} into {}", fold_against, row_index);
					for x in 0..width {
						target_row[x] = *source_row.get(x).unwrap_or(&false) || target_row[x]
					}
				}
				folded_map_rows
			}
		};
		
		let next_map = Map{
			nodes: folded_map_rows,
			fold: prev_map.fold[1..prev_map.fold.len()].to_vec() };
		//println!("{}", next_map);
		//println!("fold by {:?}", fold_instruction);
		let sum_row = |row: &Vec<bool>| -> usize { row.iter().map(|&val| if val { 1 } else { 0 }).sum() };
		let num_dots : usize = next_map.nodes.iter().map(sum_row).sum();
		println!("resultant map height: {}", next_map.nodes.len());
		println!("resultant width: {}", next_map.nodes[0].len());
		println!("resultant dot count: {}", num_dots);
		folded_maps.push(next_map);
	}
	
	folded_maps.last().unwrap().clone()
}

pub fn run() {
	let map = read_grid();
	//println!("{}", map);
	println!("first map height: {}", map.nodes.len());
	println!("first map width: {}", map.nodes[0].len());
	
	for fold_by in &map.fold {
		println!("Fold by {:?}", fold_by);
	}

	let final_map = process_folds(&map);
	
	
	println!("final map:");
	println!("{}", final_map);
}
