pub fn str_len(s: &str) -> usize {
	let mut l:usize = 0;

	for _ in s.chars() {
		l += 1;
	}

	return l;
}