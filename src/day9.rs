use std::io;
use std::collections::HashSet;

const RADIX: u32 = 10;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
struct Point {
	x: usize,
	y: usize
}

struct Basin {
	lowest_point: Point,
	size: usize
}

pub fn run() {

	let mut height_map: Vec<Vec<i32>> = Vec::new();	
	let mut width: Option<i32> = None;
	loop {
		let mut input = String::new();
		println!("Enter the input:");
		io::stdin()
			.read_line(&mut input)
			.expect("Failed to read line");
		
		if input.trim().is_empty() {
			break;
		}
	
		let row_heights: Vec<i32> = input.trim().chars().map(|c| c.to_digit(RADIX).expect("Invalid digit") as i32).collect();
		
		match width {
			None => width = Some(row_heights.len().try_into().unwrap()),
			Some(this_len) => assert!(width.unwrap() == this_len, "Inconsistent row widths")
		}
		
		height_map.push(row_heights);
	}
	
	let height = height_map.len();
	let width = width.unwrap();
	println!("Height map dims {}x{}", width, height);	
	
	let mut basins: Vec<Basin> = Vec::new();
	let mut total_risk_level = 0;
	for (y, row) in height_map.iter().enumerate() {
		for (x, h) in row.iter().enumerate() {
			
			let lh = if x > 0 { row[x-1] } else { i32::MAX };
			let rh = if x < (width-1).try_into().unwrap() { row[x+1] } else { i32::MAX };
			let uh = if y > 0 { height_map[y-1][x] } else { i32::MAX };
			let dh = if y < (height-1).try_into().unwrap() { height_map[y+1][x] } else { i32::MAX };
			
			print!("{}", h);
			if h < &lh && h < &rh && h < &uh && h < &dh {
				print!("!");
				let risk_level = 1+h;
				total_risk_level += risk_level;
				basins.push( Basin{ lowest_point: Point{x, y}, size: 0 } );
			} else {
				print!(" ");
			}
			
		}
		print!("\n");
	}
	println!("Found {} low point(s)", basins.len());
	println!("TOTAL RISK LEVEL: {}", total_risk_level);
	
	// Find out how big basins are using flood fill
	// Basins are all points flowing down to the low point, except for 9-height points.
	
	for basin in basins.iter_mut() {
		let mut visited_points = HashSet::<Point>::new();
		let mut point_queue: Vec<Point> = vec![basin.lowest_point];
		while !point_queue.is_empty() {
			let p = point_queue.pop().expect("Failed to pop from non-empty vec?!");
			if visited_points.contains(&p) { continue; };
			visited_points.insert(p);

			let point_height = height_map[p.y][p.x];
			if point_height == 9 { continue; };
			
			basin.size += 1;
			
			let mut add_point = |x: usize, y: usize| {
				if height_map[y][x] > point_height {
					point_queue.push( Point{ x, y } );
				}
			};
			
			if p.x > 0 { add_point(p.x - 1, p.y); }
			if p.x < (width-1).try_into().unwrap() { add_point(p.x + 1, p.y); }
			if p.y > 0  { add_point(p.x, p.y - 1); }
			if p.y < (height-1).try_into().unwrap() { add_point(p.x, p.y + 1); }
		}
		
		println!("Low point {:?} basin size {}", basin.lowest_point, basin.size);
	}
	
	basins.sort_by(|basin1, basin2| basin1.size.partial_cmp(&basin2.size).unwrap());
	
	let mut biggest_3_prod = 1;
	for basin in basins.iter().rev().take(3) {
		println!("Basin sorted size: {}", basin.size);
		biggest_3_prod *= basin.size;
	}
	println!("Product of 3 largest: {}", biggest_3_prod);

}