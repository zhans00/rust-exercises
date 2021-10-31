pub fn delete_and_backspace(s: &mut String) {
	let mut i = 0;
	let mut start : usize;
	let mut chars : usize;

	while i < s.len() {
		// println!("{}", s.len());
		start = i;
		while i < s.len() && s.chars().nth(i).unwrap()== '-' {
				i += 1;
		}
		chars = i - start;
		if chars != 0 {
			s.replace_range(i - 2 * chars..i, "");
			i = i - 2 * chars;
			// println!("- {} {}", s, i);
		}

		start = i;
		while i < s.len() && s.chars().nth(i).unwrap()== '+' {
				i += 1;
		}
		chars = i - start;
		if chars != 0 {
			// println!("{}", chars);
			s.replace_range(start..start + 2 * chars, "");
			i = i - 2 * chars;
			// println!("{}", s);
		}
		i += 1;
	}
}

pub fn is_correct(v: &mut Vec<&str>) -> usize {
	let mut num1 : String = String::new();
	let mut nums : Vec<i64> = Vec::new();
	let v2 : Vec<&str> = v.clone();
	let mut typ : i8 = -1;
	let mut res : i64 = 1;
	let mut corr = 0;
	let l = v.len();
	v.clear();

	for w in v2 {
		num1 = "".to_string();
		nums.clear();
		for i in w.chars() {
			if i >= '0' && i <= '9' {
				num1.push(i);
			} else if i == '=' {
				nums.push(num1.parse().unwrap());
				num1 = "".to_string();
			}
			else if i == '-' {
				nums.push(num1.parse().unwrap());
				num1 = "".to_string();
				typ = 0;

			} else if i == '+' {
				nums.push(num1.parse().unwrap());
				num1 = "".to_string();
				typ = 1;
			} else if i == '*' {
				nums.push(num1.parse().unwrap());
				num1 = "".to_string();
				typ = 2;
			} else if i == '/' {
				nums.push(num1.parse().unwrap());
				num1 = "".to_string();
				typ = 3;
			} else {
				nums.push(num1.parse().unwrap());
				num1 = "".to_string();
				typ = 4;
			}
		}

		nums.push(num1.parse().unwrap());
		if typ == 0 { 
			res = nums[0] - nums[1];
		} else if typ == 1 {
			res = nums[0] + nums[1];
		} else if typ == 2 {
			res = nums[0] * nums[1];
		} else if typ == 3 {
			res = nums[0] / nums[1];
		} else {
			res = nums[0] % nums[1];
		}

		if res == nums[2] {
			corr += 1;
			v.push("✔");
		} else {
			v.push("✘");
		}
	}

	return corr * 100 / l  as usize;
}