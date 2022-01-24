use std::io;
use std::fmt;
use std::str::FromStr;
use std::collections::HashMap;
use std::collections::HashSet;

fn read_line() -> Option<String> {
	let mut input = String::new();
	println!("Enter the input:");
	io::stdin()
		.read_line(&mut input)
		.expect("Failed to read line");
	
	match input.trim() {
		"" => None,
		input_trimmed => Some(input_trimmed.to_string())
	}
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Node {
	Start,
	End,
	SmallCave(String),
	BigCave(String)
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Node::Start => write!(f, "start").expect("Bad write"),
			Node::End => write!(f, "end").expect("Bad write"),
			Node::SmallCave(name) => write!(f, "{}", name).expect("Bad write"),
			Node::BigCave(name) => write!(f, "{}", name).expect("Bad write")
		};
		Ok(())
    }
}


impl FromStr for Node {

    type Err = ();

    fn from_str(input: &str) -> Result<Node, Self::Err> {
		match input.trim() {
			"start" => Ok(Node::Start),
			"end" => Ok(Node::End),
			name => {
				// TODO: technically we should validate all chars here, could probably do elegantly with some iterator/mapping thing
				if name.chars().next().unwrap().is_ascii_uppercase() {
					Ok( Node::BigCave(name.to_string()) )
				} else {
					Ok( Node::SmallCave(name.to_string()) )
				}
			}			
		}
    }
}

#[derive(Debug)]
struct Edge (Node, Node);

impl FromStr for Edge {

    type Err = ();

    fn from_str(input: &str) -> Result<Edge, Self::Err> {
		
		let mut items = input.trim().split("-");//.collect::<Vec<&str>>() {
		
		// TODO full error handling rather than assert
		let from_node = items
			.next()
			.expect("Bad format for edge")
			.parse::<Node>()
			.expect("Bad format for node");
			

		let to_node = items
			.next()
			.expect("Bad format for edge")
			.parse::<Node>()
			.expect("Bad format for node");
		
		assert!(items.next() == None, "Bad format for edge");
		
		Ok( Edge(from_node, to_node) )
    }
}

#[derive(Debug)]
struct Map {
	// Map node to adjacent nodes
	nodes: HashMap< Node, Vec<Node> >
}

fn read_map() -> Map {
	let mut nodes : HashMap< Node, Vec<Node> > = HashMap::new();
	
	loop {
		match read_line() {
			Some(input) => {
				// TODO this isn't ideal as it may do an unnecessary clone..
				// In principle if we could get at the hash we wouldn't need entry to borrow?
				// Is there an elegant way of avoiding this?
				// See https://github.com/rust-lang/rust/issues/51604 and related discussions
				let edge = input.parse::<Edge>().unwrap();
				{
					let adjacency_0 = nodes
						.entry(edge.0.clone())
						.or_insert(Vec::new());
					adjacency_0.push(edge.1.clone());
				}
					
				{
					let adjacency_1 = nodes
						.entry(edge.1)
						.or_insert(Vec::new());
					adjacency_1.push(edge.0);
				}
			},
			None => break
		}
	}
	
	// Do basic validation
	assert!(nodes.contains_key(&Node::Start));
	assert!(nodes.contains_key(&Node::End));
	
	Map{ nodes }
}

fn can_visit_once(path: &Vec<Node>, test_node: &Node) -> bool {
	match test_node {
		Node::Start => false,
		Node::End => true,
		Node::SmallCave(_) => !path.contains(test_node),
		Node::BigCave(_) => path.last().unwrap() != test_node
	}
}

fn can_visit_twice(path: &Vec<Node>, test_node: &Node) -> bool {
	match test_node {
		Node::Start => false,
		Node::End => true,
		Node::SmallCave(_) => {
			// We can only visit a small cave again
			// if we haven't already visited any small cave twice
			if !path.contains(test_node) {
				true
			} else {
				let mut uniq = HashSet::new();
				path.into_iter().filter(|n| matches!(n, Node::SmallCave(_))).all(move |x| uniq.insert(x))
			}
		},
		Node::BigCave(_) => path.last().unwrap() != test_node
	}
}

fn list_paths(map: &Map, can_visit_fn: fn(&Vec<Node>, &Node) -> bool) {
	let mut partial_paths : Vec< Vec<Node> > = vec![ vec![Node::Start] ];
	let mut complete_paths : Vec< Vec<Node> > = Vec::new();
	
	while !partial_paths.is_empty() {
		// Get next path to process.
		let path = partial_paths.pop().unwrap();
		if path.last().unwrap() == &Node::End {
			// A complete path just goes straight into the completed list.
			complete_paths.push(path);
		} else {
			// For an incomplete path, let's add a new incomplete path for each adjacent node
			// But avoid backtracking (naively)			
			
			//println!("try extend partial path {:?}...", path);
			let adjacent_nodes = &map.nodes[&path.last().unwrap()];
			for next_node in adjacent_nodes.into_iter().filter(|n| can_visit_fn(&path, n)) {
				let mut extended = path.clone();
				extended.push(next_node.clone());
				//println!("    try extend with {:?} -> {:?}", next_node, extended);
				partial_paths.push(extended);
			}
		}	
	}
	// HACK TODO need to double-check this is no longer needed
	complete_paths.dedup();
	
	println!("Found path(s):");
	for path in &complete_paths {
		print!("    {}", path[0]);
		for e in &path[1..] {
			print!(",{}", e);
		}
		print!("\n");
	}
	println!("Total path count: {}", complete_paths.len());
}

pub fn run() {

	let map = read_map();
	println!("map: {:?}", map);
	println!("LIST PATHS FOR VISIT_ONCE");
	list_paths(&map, can_visit_once);
	println!("LIST PATHS FOR VISIT_TWICE");
	list_paths(&map, can_visit_twice);
}
