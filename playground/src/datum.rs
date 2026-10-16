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
mod test {
	use super::*;
	#[test]
	fn new() {
		let fixture = Output::new('a');
		assert_eq!(fixture.1, 1);
		assert_eq!(fixture.0, [b'a', 0, 0, 0]);

		let fixture = Output::new('🥺');
		let mut buf = [0u8; 4];
		let expected = '🥺'.encode_utf8(&mut buf).as_bytes();
		assert_eq!(fixture.1, 4);
		assert_eq!(&fixture.0, &expected);
	}

	#[test]
	fn datum() {
		let fixture = Output::new('a');
		assert_eq!(fixture.datum(), &[b'a']);
		assert_eq!(fixture.datum().iter().len(), 1);

		let fixture = Output::new('🥺');
		let mut buf = [0u8; 4];
		let expected = '🥺'.encode_utf8(&mut buf).as_bytes();
		assert_eq!(fixture.datum(), expected);
		assert_eq!(fixture.datum().iter().len(), 4);
	}

	#[test]
	fn len() {
		let fixture = Output::new('a');
		assert_eq!(fixture.len(), 1);

		let fixture = Output::new('🥺');
		assert_eq!(fixture.len(), 4);
	}
}
