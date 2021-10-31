use std::collections::HashMap;

pub fn bigger(h: HashMap<&str, i32>) -> i32 {
    let mut hash_vec: Vec<_> = h.iter().collect();
    hash_vec.sort_by(|a, b| b.1.cmp(a.1));
    let t = hash_vec[0].1;
    let res: i32 = *t;
    res
}
