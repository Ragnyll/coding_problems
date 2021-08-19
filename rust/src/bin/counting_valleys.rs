use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .ok()
        .expect("Failed to read line");

    let mut in_valley: bool = false;
    let mut current_level: i32 = 0;
    let mut num_valleys: usize = 0;

    for character in input.chars() {
        match character {
            'U' => {
                current_level += 1;
                if in_valley == true && current_level == 0 {
                    num_valleys += 1;
                    in_valley = false;
                }
            },
            'D' => {
                current_level -= 1;
                if current_level >= 0 {
                    in_valley = false;
                } else {
                    in_valley = true;
                }
            },
            _ => (),
        }
    }

    println!("{}", num_valleys);
}
