pub fn first_subword(mut s: String) -> String {
    let snake = '_';
    let mut capital = true;
    let mut capital_found = false;
    let mut ans = String::new();

    if s.chars().nth(0).unwrap() >= 'a' && s.chars().nth(0).unwrap() <= 'z' {
        capital = false;
    }

    for c in s.chars() {
        if c == snake {
            return ans;
        }

        if capital && !capital_found && c >= 'A' && c <= 'Z' {
            capital_found = true;
        }

        else if capital && capital_found && c >= 'A' && c <= 'Z' {
            return ans;
        }

        if !capital && c >= 'A' && c <= 'Z' {
            return ans;
        }
        ans.push(c);
    }

    return ans;
}