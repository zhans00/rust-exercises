pub fn sum(a: &[i32]) -> i32 {
	let mut sum = 0;

	for i in 0..10 {
		sum += a[i];
	}

	return sum;
}

pub fn thirtytwo_tens() -> [i32; 32] {
	return [10; 32];
}