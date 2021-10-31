pub fn nbr_function(c: u32) -> (u32, f64, f64) {
    let c2 = c as f64;
    let exp_f = c2.exp();
    let log = c2.ln().abs();
    return (c, exp_f, log);
}

pub fn str_function(a: String) -> (String, String) {
    let mut nums:String = String::new();
    let mut num:String = String::new();

    for c in a.chars() {
        if c == ' ' {
            let num2: f64 = num.parse().unwrap();
            nums.push_str(&num2.exp().to_string());
            nums.push(' ');
            num = "".to_string();
        } else {
            num.push(c);
        }
    }

    let num2: f64 = num.parse().unwrap();
    nums.push_str(&num2.exp().to_string());

    return (a, nums);
}

pub fn vec_function(b: Vec<u32>) -> (Vec<u32>, Vec<f64>) {
    let mut nums:Vec<f64> = Vec::new();

    for c in 0..b.len() {
        let c2 = b[c] as f64;
        nums.push(c2.ln().abs());

    }

    return (b, nums);
}
