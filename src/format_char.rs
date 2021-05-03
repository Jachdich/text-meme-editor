use crate::rgb::RGB;

#[derive(Copy, Clone, Debug)]
pub struct FmtChar {
	pub ch: char,
	pub fg: RGB,
	pub bg: RGB,
}