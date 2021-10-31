pub fn doubtful(s: &mut String) {
	s.push('?');
}

pub fn takes_ownership(some_string: i32) { // some_string comes into scope
	println!("{}", some_string);
}
