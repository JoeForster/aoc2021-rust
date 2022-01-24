use std::io;


const DEFAULT_NUM_DAYS : u32 = 18;
const INITIAL_TIMER : usize = 8;
const MAP_SIZE : usize = (INITIAL_TIMER as usize)+1;
const RESET_TIMER : usize  = INITIAL_TIMER-2;

fn run_spawn_step(spawn_timers: &mut Vec<u8>) {

	let mut new_spawns = Vec::<u8>::new();
	for timer in spawn_timers.iter_mut() {
		if *timer == 0 {
			new_spawns.push(INITIAL_TIMER as u8);
			*timer = RESET_TIMER as u8;
		} else {
			*timer -= 1;
		}
	}
	spawn_timers.append(&mut new_spawns);
}

fn run_spawn_step_mapping(spawn_timer_to_count: &mut [u64; MAP_SIZE]) {

	// We will spawn one for every fish at timer 0
	let num_to_spawn = spawn_timer_to_count[0];
	
	// Count down all ages 1+
	for timer_index in 1..MAP_SIZE  {
		spawn_timer_to_count[timer_index-1] = spawn_timer_to_count[timer_index];
	}
	// New spawns replace the number at age INITIAL_TIMER (all of which now have age INITIAL_TIMER-1)
	spawn_timer_to_count[INITIAL_TIMER] = num_to_spawn;
	// The ones that just spawned are added to the total count at RESET_TIMER
	spawn_timer_to_count[RESET_TIMER] += num_to_spawn;
}

pub fn run() {

	let mut input = String::new();
	println!("Enter the input:");
	io::stdin()
		.read_line(&mut input)
		.expect("Failed to read line");
		
	
	let mut spawn_timers: Vec<u8> = input
		.trim()
		.split(",")
        .into_iter()
        .map(|s| s.parse::<u8>().expect("Invalid u32 in input list"))
        .collect();

	println!("Enter number of days (default: {})", DEFAULT_NUM_DAYS);
	input = String::new();
	io::stdin()
		.read_line(&mut input)
		.expect("Failed to read line");	
	let num_days : u32 = match input.trim().parse() {
		Ok(n) => n,
		Err(_) => DEFAULT_NUM_DAYS
	};

	println!("Choose method 1=vector 2=map (default: map)");
	input = String::new();
	io::stdin()
		.read_line(&mut input)
		.expect("Failed to read line");	
	let use_map : bool = match input.trim().parse() {
		Ok(1) => false,
		_ => true
	};


	println!("Initial state: {:?}", spawn_timers);

	if use_map {
		
		let mut spawn_timer_to_count : [u64; MAP_SIZE] = [0; MAP_SIZE];
		for initial_timer in spawn_timers {
			assert!(initial_timer as usize <= MAP_SIZE);
			spawn_timer_to_count[initial_timer as usize] += 1;
		}
		for day_num in 1..=num_days {
			run_spawn_step_mapping(&mut spawn_timer_to_count);
			//println!("After {} days: {:?}", day_num, spawn_timers);
			let cur_count : u64 = spawn_timer_to_count.iter().sum();
			println!("After {} days, the counts are: {:?} (total {})", day_num, spawn_timer_to_count, cur_count);
		}
			let cur_count : u64 = spawn_timer_to_count.iter().sum();
		println!("FINAL count: {}", cur_count);
		
	} else {
		for day_num in 1..=num_days {
			run_spawn_step(&mut spawn_timers);
			//println!("After {} days: {:?}", day_num, spawn_timers);
			println!("After {} days, the count is: {}", day_num, spawn_timers.len());
		}
		println!("FINAL count: {}", spawn_timers.len());
	}


	
	//
}