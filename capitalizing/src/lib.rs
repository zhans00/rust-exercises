pub fn capitalize_first(input: &str) -> String {
	let mut output = String::from(input);

	for i in 0..input.len() {
		let ch = input.chars().nth(i).unwrap();
		if ch.is_alphabetic() {
			if ch >= 'a' && ch <= 'z' {
				let rep : char = (ch as u8 - 32) as char;
				output.replace_range(i..i + 1, rep.to_string().as_str());
			} 
			return output;
		}
	}

	return output;
}

pub fn title_case(input: &str) -> String {
	let mut output = String::from(input);
	let mut word = true;

	for i in 0..input.len() {
		let ch = input.chars().nth(i).unwrap();

		if ch.is_alphabetic() && word && ch >= 'a' && ch <= 'z' {
			let rep = (ch as u8 - 32) as char;
			output.replace_range(i..i+1, rep.to_string().as_str());
			word = false;
		} else if ch.is_whitespace() {
			word = true;
		}
	}

	return output;
}

pub fn change_case(input: &str) -> String {
	let mut output = String::from(input);

	for i in 0..input.len() {
		let ch = input.chars().nth(i).unwrap();

		if ch.is_alphabetic() && ch >= 'a' && ch <= 'z' {
			let rep = (ch as u8 - 32) as char;
			output.replace_range(i..i+1, rep.to_string().as_str());
		} else if ch.is_alphabetic() && ch >= 'A' && ch <= 'Z' {
			let rep = (ch as u8 + 32) as char;
			output.replace_range(i..i+1, rep.to_string().as_str());
		}
	}

	return output;
}