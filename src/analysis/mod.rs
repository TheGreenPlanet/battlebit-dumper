mod misc;

pub fn parse(o: &mut String, image: &[u8]) {
	use pelite::pe64::*;
	let bin = PeFile::from_bytes(image).unwrap();
	misc::print(o, bin);
}
