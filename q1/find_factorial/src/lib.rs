pub fn factorial(num: u64) -> u64 {
	if num == 0 || num == 1 { return 1; }

	let mut ans: u64 = 1;

	for i in 2..num + 1 {
		ans *= i;
	}

	return ans;
}