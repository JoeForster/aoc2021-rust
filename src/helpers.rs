use std::io::{self, BufRead, Read};
use std::fs::File;

pub struct FileReader {
	reader: io::BufReader<File>,
}

impl FileReader {
	pub fn open(path: impl AsRef<std::path::Path>) -> io::Result<Self> {
		let file = File::open(path)?;
		let reader = io::BufReader::new(file);

		Ok(Self { reader })
	}

	pub fn read_line<'buf>(
		&mut self,
		buffer: &'buf mut String,
	) -> Option<io::Result<&'buf mut String>> {
		buffer.clear();
		// TODO may be best to try and strip out endlines here?
		self.reader
			.read_line(buffer)
			.map(|u| if u == 0 { None } else { Some(buffer) })
			.transpose()
	}
}


//pub fn read_line(day: u32) -> Option<String> {
//	
//	let file_name = format!("inputs/day{}.txt", day);
//	let file_opt = File::open(filename);
//	match file_opt {
//		Some(file) => {
//			
//		},
//		_ => {
//		let mut input = String::new();
//			println!("File '{}' couldn't be read! Enter the input:", file_name);
//			io::stdin()
//				.read_line(&mut input)
//				.expect("Failed to read line");
//			
//			match input.trim() {
//				"" => None,
//				input_trimmed => Some(input_trimmed.to_string())
//			}
//		}
//	}
//}

// Quick helper for reading answers. Liable to change when generalised to all days.
// TODO at least return an option, which we'll need for incomplete days..?
pub fn read_answer(day: u32, part: u32) -> String {
	let mut answer_str = String::new();
	let mut answer_file = File::open(format!("answers/day{}_part{}.txt", day, part))
		.expect("Couldn't read answer file");
	answer_file.read_to_string(&mut answer_str).expect("Failed to read string");
	return answer_str;
}