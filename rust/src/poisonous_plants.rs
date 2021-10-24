// There are a number of plants in a garden. Each of the plants has been treated with some amount of pesticide. After each day, if any plant has more pesticide than the plant on its left, being weaker than the left one, it dies.

// You are given the initial values of the pesticide in each of the plants. Determine the number of days after which no plant dies, i.e. the time after which there is no plant with more pesticide content than the plant to its left.
//

/// Determines if two arrays are the same
fn is_same(a: &[i32], b: &[i32]) -> bool {
    if a.len() != b.len() {
        return false;
    }

    for i in 0..a.len() {
        if a[i] != b[i] {
            return false;
        }
    }
    true
}

/// Given an array of pesticide_lvls determines the number of days after which no plant dies
#[allow(dead_code)]
fn poisonous_plants(pesticide_lvls: &[i32]) -> i32 {
    if pesticide_lvls.len() == 0 || pesticide_lvls.len() == 1 {
        return 1;
    }
    let mut days_survived = -1;

    let mut surviving_plants_current: Vec<i32> = Vec::with_capacity(pesticide_lvls.len());
    let mut surviving_plants_previous = pesticide_lvls.to_vec();


    while !is_same(&surviving_plants_current, &surviving_plants_previous) {
        surviving_plants_current = surviving_plants_previous.clone();
        surviving_plants_previous = Vec::with_capacity(surviving_plants_current.len());

        println!("surviving_plants current before loop: {:?}", surviving_plants_current);
        for i in 0..surviving_plants_current.len() {
            // cant look left. leftmost never dies based on description
            if i == 0 {
                surviving_plants_previous.push(surviving_plants_current[i]);
                continue;
            }

            // the current plant survives
            if surviving_plants_current[i] < surviving_plants_current[i - 1] {
                surviving_plants_previous.push(surviving_plants_current[i]);
            }
        }

        println!("surviving_plants current: {:?}", surviving_plants_current);
        println!("surviving_plants previous: {:?}", surviving_plants_previous);

        days_survived += 1;
    }

    days_survived
}

#[cfg(test)]
pub mod test {
    use super::poisonous_plants;

    #[test]
    fn test_case1() {
        // empty case
        assert_eq!(1, poisonous_plants(&[]));
    }

    #[test]
    fn test_case2() {
        // only one plant
        assert_eq!(1, poisonous_plants(&[0]));
    }

    #[test]
    fn test_case3() {
        assert_eq!(2, poisonous_plants(&[6, 5, 8, 4, 7, 10, 9]));
    }

    #[test]
    fn test_case4() {
        // all plants to the right will die
        assert_eq!(1, poisonous_plants(&[1, 2, 3, 4, 5, 6]));
    }
}
