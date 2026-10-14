pub struct LoopIterator(Vec<u8>,usize);

impl LoopIterator{
	pub fn new(scr:Vec<u8>)->Self{
		Self(scr,0)
	}
	
	pub fn from_str(scr:&str)->Self{
		Self::new(scr.as_bytes().into())
	}
	
	pub fn next(&mut self)->u8{
		if self.1==self.0.len(){
			self.1=0;
			self.0[self.1]
		} else {
			let result = self.0[self.1];
			self.1 += 1;
			result
		}
	}
}