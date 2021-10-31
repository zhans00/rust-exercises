pub fn char_length(s: &str) -> usize {
	let mut l = 0;
	for _ in s.chars() {
		l += 1;
	}

	return l;
}