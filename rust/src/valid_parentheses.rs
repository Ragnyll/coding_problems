use std::collections::HashMap;

pub fn is_valid(s: String) -> bool {
    let mut o_c_map = HashMap::with_capacity(3);
    o_c_map.insert(')', '(');
    o_c_map.insert('}', '{');
    o_c_map.insert(']', '[');
    let mut stack = vec![];

    for c in s.chars() {
        if o_c_map.contains_key(&c) {
            match stack.pop() {
                Some(e) => {
                    if o_c_map[&c] != e {
                        return false;
                    }
                }
                None => return false,
            }
        } else {
            stack.push(c);
        }
    }

    stack.is_empty()
}
