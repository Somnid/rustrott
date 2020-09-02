use std::io::{Read};
use std::convert::{TryInto};

pub fn read_u32(file: &mut dyn Read) -> u32 {
	let mut buf: [u8; 4] = [0; 4];
	let _ = file.take(4).read(&mut buf);
	u32::from_le_bytes(buf)
}

pub fn u32_from_slice(bytes: &[u8], index: usize) -> u32 {
	u32::from_le_bytes(bytes[index..index+4].try_into().unwrap())
}

pub fn str_from_slice(bytes: &[u8], index: usize, len: usize) -> String {
	String::from_utf8_lossy(&bytes[index..index+len]).to_string()
}

pub fn read_str(file: &mut dyn Read, len: usize) -> String {
	let mut buf = vec![0; len];
	let _ = file.take(len as u64).read(&mut buf);
	String::from_utf8_lossy(&buf).to_string()
}