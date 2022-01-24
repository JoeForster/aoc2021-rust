use std::io;
use std::str::FromStr;

enum Command {
	Forward(u32),
	Down(u32),
	Up(u32)
}

impl FromStr for Command {

    type Err = ();

    fn from_str(input: &str) -> Result<Command, Self::Err> {
		let values = input.split(" ");
		let values = values.collect::<Vec<&str>>();

        let dist = match values.len() {
			2 => values[1].trim(),
			_ => return Err(())
        };
		
		let dist: u32 = match dist.parse() {
			Ok(dist) => dist,
			Err(_) => return Err(())
		};
		
        match values[0] {
            "forward"  	=> Ok(Command::Forward(dist)),
            "down"  	=> Ok(Command::Down(dist)),
            "up"  		=> Ok(Command::Up(dist)),
            _      		=> Err(()),
        }
    }
}

fn day2_part1(v: &Vec<Command>) {
	
	let mut x_pos = 0;
	let mut depth = 0;
	for cmd in v {
        match cmd {
            Command::Forward(dist) 	=> x_pos = x_pos + dist,
            Command::Down(dist) 	=> depth = depth + dist,
            Command::Up(dist) 		=> depth = depth - dist,
        }
	}
	let dist_sq = x_pos * depth;
	println!("Final position: x_pos {} depth {} dist_sq is {}", x_pos, depth, dist_sq);

}


fn day2_part2(v: &Vec<Command>) {
	
	let mut x_pos = 0;
	let mut depth = 0;
	let mut aim = 0;
	for cmd in v {
        match cmd {
            Command::Forward(dist) 	=> { x_pos = x_pos + dist; depth = depth + aim * dist },
            Command::Down(dist) 	=> aim = aim + dist,
            Command::Up(dist) 		=> aim = aim - dist,
        }
	}
	let dist_sq = x_pos * depth;
	println!("Final position: x_pos {} depth {} dist_sq is {}", x_pos, depth, dist_sq);

}


pub fn run() {

	let mut v: Vec<Command> = Vec::new();
	loop {
		let mut input = String::new();
		println!("Type something:");

		io::stdin()
			.read_line(&mut input)
			.expect("Failed to read line");

		println!("Here is the thing you typed: {}", input);
		
		let cmd = match Command::from_str(&input) {
            Ok(cmd) => cmd,
            Err(_) => break,
        };
		
		v.push(cmd);

	}
	//println!("Here are the values you typed:");
	//for n in &v {
	//	println!("'{}'", n);
	//}
	
	day2_part1(&v);

	day2_part2(&v);
}
