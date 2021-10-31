pub fn initials(names: Vec<&str>) -> Vec<String> {
	let mut ans:Vec<String> = Vec::new();

	for n in names {
		let mut split = n.split_whitespace();
		let mut f : String = String::new();
		f.push(split.next().unwrap().chars().nth(0).unwrap());
		f.push_str(". ");
		f.push(split.next().unwrap().chars().nth(0).unwrap());
		f.push('.');
		ans.push(f);
		
	}

	return ans;
}