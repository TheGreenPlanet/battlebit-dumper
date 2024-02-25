use std::fmt;

mod misc;

#[derive(Default)]
pub struct Output {
	pub ini: String,
	pub human: String,
}

// Nicely format supposedly valid identifier-like strings
fn ident(s: &str) -> impl '_ + fmt::Display {
	fmtools::fmt! { move
		if s.is_empty() { "{empty}" }
		else if s.contains("\"") || s.contains(" ") || s.contains("\n") { {s:?} }
		else { {s} }
	}
}

pub fn parse(f: &mut Output, image: &[u8]) {
	use pelite::pe64::*;
	let bin = PeFile::from_bytes(image).unwrap();
	misc::print(f, bin);
}
