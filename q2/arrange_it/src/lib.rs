use std::collections::HashMap;

pub fn arrange_phrase(phrase: &str) -> String {
	let mut split = phrase.split_whitespace();
	let mut w = split.next();
	let mut array = HashMap::new();
	let mut l = 0;
	let mut ans : String = String::new();

	while w != None {
		let mut num : String = String::new();
		let mut word : String = String::new();

		for i in w.unwrap().chars() {
			if i >= '0' && i <= '9' {
				num.push(i);
			} else {
				word.push(i);
			}
		}
		let num2: usize = num.parse().unwrap();
		array.insert(num2, word);
		w = split.next();

		l += 1;
	}

	for i in 1 as usize..l + 1 {
		ans.push_str(array.get(&i).unwrap());
		if i != l {
		ans.push(' ');
		}
	}
	return ans;
}