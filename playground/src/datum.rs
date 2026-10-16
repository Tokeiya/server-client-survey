#[derive(Clone, Copy)]
struct Output([u8; 4], u8);

impl Output {
	pub fn new(scr: char) -> Self {
		let mut buff = [0u8; 4];
		let len = scr.encode_utf8(&mut buff).len();
		Self(buff, len as u8)
	}

	pub fn datum(&self) -> &[u8] {
		&self.0[..self.1 as usize]
	}

	pub fn len(&self) -> usize {
		self.1 as usize
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn foo() {
		todo!();
	}
}
