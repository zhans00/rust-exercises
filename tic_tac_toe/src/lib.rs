pub fn tic_tac_toe(table: Vec<Vec<&str>>) -> String {
	// println!("{}",diagonals("X", &table) );
	if diagonals("X", &table) || horizontal("X", &table) || vertical("X", &table) {
		return "player X won".to_string();
	}

	if diagonals("O", &table) || horizontal("O", &table) || vertical("O", &table) {
		return "player O won".to_string();
	}

	return "Tie".to_string();

}

pub fn diagonals(player: &str, table: &Vec<Vec<&str>>) -> bool {
	let mut i:usize = 0;
	let mut j :usize = 0;
	let mut toe = true;

	while i < table.len() {
		if table[i][j] != player {
			toe = false;
		}

		i += 1;
		j += 1;
	}

	if toe {
		return true;
	}
	i = 0;
	j = table[0].len() - 1;

	while i < table[0].len() {
		if table[i][j] != player {
			return false;
		}

		if j == 0 { break; }
		i += 1;
		j -= 1;
	}

	return true;
}

pub fn horizontal(player: &str, table: &Vec<Vec<&str>>) -> bool {
	let mut ans:bool;
	for i in 0..table.len() {
		ans = true;
		for j in 0..table[0].len() {
			if table[i][j] != player {
				ans = false;
				break;
			}
		}

		if ans {
			return true;
		}
	}

	return false;
}

pub fn vertical(player: &str, table: &Vec<Vec<&str>>) -> bool {
	let mut ans:bool;
	for j in 0..table[0].len() {
		ans = true;
		for i in 0..table.len() {
			if table[i][j] != player {
				ans = false;
				break;
			}
		}

		if ans {
			return true;
		}
	}

	return false;
}