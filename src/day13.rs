
//use std::fmt;
//use std::str::FromStr;
//use std::collections::HashMap;
//use std::collections::HashSet;

// TODO to impl display
//struct Row {
//	vals: Vec<bool>
//}

//use crate::helpers::helpers::read_line;
use crate::helpers::FileReader;

#[derive(Debug)]
struct Map {
	nodes: Vec< Vec<bool> >
}


fn read_grid() -> Map {
	let mut nodes : Vec< Vec<bool> > = Vec::new();
	
	let mut width : usize = 0; 
	let mut height : usize = 0; 
	let mut buffer = String::new();
	
	let mut file_reader = FileReader::open("day13_input.txt").expect("Failed to open file");
	
	while let Some(line) = file_reader.read_line(&mut buffer) {
		println!("Buffer: {}", buffer);
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

	Map{ nodes }
}

pub fn run() {
	let map = read_grid();
	println!("{:?}", map);

}
