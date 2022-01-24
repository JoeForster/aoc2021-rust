use std::io;
use std::str::FromStr;
use std::collections::HashMap;

// TODO name props x,y?
#[derive(Debug)]
struct Point (u32, u32);

impl FromStr for Point {

    type Err = ();

    fn from_str(input: &str) -> Result<Point, Self::Err> {
		let values = input.trim().split(",");
		let values = values.collect::<Vec<&str>>();
		if values.len() != 2 {
			return Err(());
		}
		match (values[0].parse(), values[1].parse()) {
			(Ok(x), Ok(y)) => Ok(Point(x, y)),
			_ => Err(())
		}
    }
}

#[derive(Debug)]
struct Line (Point, Point);

impl FromStr for Line {

    type Err = ();

    fn from_str(input: &str) -> Result<Line, Self::Err> {
		let values = input.trim().split("->");
		let values = values.collect::<Vec<&str>>();
		if values.len() != 2 {
			return Err(());
		}
		match (values[0].parse(), values[1].parse()) {
			(Ok(p1), Ok(p2)) => Ok(Line(p1, p2)),
			_ => Err(())
		}
    }
}

fn read_lines() -> Vec::<Line> {
	let mut lines = Vec::<Line>::new();
	loop {
		let mut input = String::new();
		println!("Enter input:");
		io::stdin()
			.read_line(&mut input)
			.expect("Failed to read line");

		if input.trim().is_empty() {
			break;
		} else {
			let line : Line = input.parse().expect("Invalid input for Line");
			lines.push(line);
		}
	}
	return lines;
}

fn count_lines(lines: &Vec::<Line>, include_diagonal: bool) -> u32 {

	let mut num_overlaps = 0;
	let mut coord_counts = HashMap::new();
	let mut max_x = 0;
	let mut max_y = 0;
	for line in lines {
		println!("Line: {:?}", line);
		
		let from_x = line.0.0;
		let from_y = line.0.1;
		let to_x = line.1.0;
		let to_y = line.1.1;
		
		if from_x > max_x {
			max_x = from_x;
		} else if to_x > max_x {
			max_x = to_x;
		}
		if from_y > max_y {
			max_y = from_y;
		} else if to_y > max_y {
			max_y = to_y;
		}
		// Horizontal lines
		if from_x == to_x {
			let x = from_x;
			let range = if to_y >= from_y { from_y..=to_y } else { to_y..=from_y };
			for y in range {
				println!("Check horizontal x:{} y:{}", x, y);
				let count = coord_counts.entry((x, y)).or_insert(0);
				*count += 1;
				if *count == 2 {
					num_overlaps += 1;
				}
			}
		}
		// Vertical lines
		else if from_y == to_y {
			let y = from_y;
			let range = if to_x >= from_x { from_x..=to_x } else { to_x..=from_x };
			for x in range {
				println!("Check vertical x:{} y:{}", x, y);
				let count = coord_counts.entry((x, y)).or_insert(0);
				*count += 1;
				if *count == 2 {
					num_overlaps += 1;
				}
			}
		}
		// Diagonal lines (ASSUMING only 45 degree lines!)
		// NOTE could probably be more optimal, and passing a bool to change behaviour like this is a bit smelly
		else if include_diagonal {
			// TODO: There's got to be a better way!
			let to_x_i : i32 = to_x.try_into().unwrap();			
			let mut x: i32 = from_x.try_into().unwrap();
			let mut y: i32 = from_y.try_into().unwrap();
			
			let x_grad : i32 = if to_x >= from_x { 1 } else { -1 };
			let y_grad : i32 = if to_y >= from_y { 1 } else { -1 };
			
			loop {
				println!("Check diagonal x:{} y:{}", x, y);
				let count = coord_counts.entry((x.try_into().unwrap(), y.try_into().unwrap())).or_insert(0);
				*count += 1;
				if *count == 2 {
					num_overlaps += 1;
				}
				if x == to_x_i {
					break;
				}
				x += x_grad;
				y += y_grad;
			}
		}
	}
	
	// Debug output the grid
	for y in 0..=max_y {
		for x in 0..=max_x {
			let count_result = coord_counts.get(&(x, y));
			match count_result {
				None => print!("."),
				Some(count) => print!("{}",count)
			}
		}
		print!("\n");
	}
	
	num_overlaps
}


pub fn run() {

	let lines = read_lines();	
	let num_overlaps_hv = count_lines(&lines, false);
	let num_overlaps_hvd = count_lines(&lines, true);

	println!("Num overlaps horizontal/vertical: {:?}", num_overlaps_hv);
	println!("Num overlaps horizontal/vertical/diagonal: {:?}", num_overlaps_hvd);

}