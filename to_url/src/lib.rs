pub fn to_url(s: &str) -> String {
	let mut url = String::new();

	for c in s.chars() {
		if c == ' ' {
			url.push_str("%20");
		} else {
			url.push(c);
		}
	}

	return url;
}