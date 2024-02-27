use std::{env, fmt};
use std::path::PathBuf;

mod analysis;

fn parse_arg() -> Option<PathBuf> {
	let mut args_os = env::args_os();
	args_os.next()?;
	let path = args_os.next().map(|path| path.into())?;
	
	Some(path)
}

pub fn print_error(error: impl fmt::Display) {
	eprintln!("{}", error);
}

fn main() {
	match parse_arg() {
		None => {
			eprintln!("Give the path to the battlebit.exe binary.");
			return;
		},
		Some(path) => {
			let filemap = pelite::FileMap::open(&path).unwrap();
			let mut output = "".to_string();
			analysis::parse(&mut output, filemap.as_ref());
			print!("{}", output);
		},
	}
}
