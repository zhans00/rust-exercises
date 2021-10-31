use std::collections::HashMap;

pub fn is_permutation(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }

    let mut s1_map = HashMap::new();
    let mut s2_map = HashMap::new();

    for i in s1.chars() {
        if s1_map.contains_key(&i) {
            *s1_map.get_mut(&i).unwrap() += 1;
        } else {
            s1_map.insert(i, 1);
            s2_map.insert(i, 0);
        }
    }

    for i in s2.chars() {
        if s2_map.contains_key(&i) {
            *s2_map.get_mut(&i).unwrap() += 1;
        } else {
            s2_map.insert(i, 1);
            s1_map.insert(i, 0);
        }
    }

    for i in s1_map.keys() {
        if s1_map[i] != s2_map[i] {
            return false;
        }
    }

    true
}
