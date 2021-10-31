pub fn fibonacci(n: u32) -> u32 {
	if n == 0 { return 0}
	
	let mut first: u32 = 1;
	let mut second: u32 = 1;

	for _ in 2..n {
		let x: u32 = first + second;
		first = second;
		second = x;
	}

	return second;
}