fn main() {
	let string = "我、地に平和を与えん為に来たと思う勿れ".to_string();
	let end_point = string.char_indices().nth(5).unwrap().0;
	let str = &string[..end_point];

	let bytes = str.as_bytes(); //This point.
}
