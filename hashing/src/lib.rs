use std::collections::HashMap;

pub fn mean(list: &Vec<i32>) -> f64 {
	let mut sum = 0;

	for i in list.iter() {
		sum += i;
	}

	return (sum as f64)/(list.len() as f64);
}

pub fn median(list: &Vec<i32>) -> i32 {
	let mut srtd = list.clone();
	srtd.sort();

	if list.len() % 2 == 0 {
		return (srtd[list.len()/2 - 1] + list[list.len()/2]) / 2;
	} else {
		return srtd[list.len() / 2]
	}
}

pub fn mode(list: &Vec<i32>) -> i32 {
	let mut map = HashMap::new();
	let mut val = list[0];
	let mut max = 1;

	for i in 0..list.len() {
		if map.contains_key(&list[i]) {
			*map.get_mut(&list[i]).unwrap() += 1;
			if map[&list[i]] > max {
				max = map[&list[i]];
				val = list[i];
			}
		} else {
			map.insert(list[i], 1);
		}
	}

	return val;
}