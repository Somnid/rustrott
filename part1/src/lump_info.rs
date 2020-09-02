use std::fmt::{Display,Formatter,Result};

pub struct LumpInfo {
	pub offset: u32,
	pub size: u32,
	pub name: String
}

impl Display for LumpInfo {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}, {}, {}", self.name, self.size, self.offset)
    }
}