use string_permutation::*;

fn main() {
	let word = "thouaght";
	let word1 = "thoxugth";
	println!(
		"Is `{}` a permutation of `{}`? = {}",
		word,
		word1,
		is_permutation(word, word1)
	);
}
