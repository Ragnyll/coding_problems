use std::collections::HashSet;

pub fn flood_fill(image: &mut Vec<Vec<i32>>, sr: i32, sc: i32, new_color: i32) -> Vec<Vec<i32>> {
    fn flood_from_index(
        image: &mut Vec<Vec<i32>>,
        r: i32,
        c: i32,
        old_color: i32,
        new_color: i32,
        visited: &mut HashSet<(i32, i32)>,
    ) {
        if image[r as usize][c as usize] == old_color && !visited.contains(&(r, c)) {
            image[r as usize][c as usize] = new_color;
            visited.insert((r, c));
            // up
            if r + 1 < image.len() as i32 {
                flood_from_index(image, r + 1, c, old_color, new_color, visited);
            }
            // down
            if r - 1 > 0 {
                flood_from_index(image, r - 1, c, old_color, new_color, visited);
            }
            // left
            if c + 1 < image[r as usize].len() as i32 {
                flood_from_index(image, r, c + 1, old_color, new_color, visited);
            }
            // right
            if c - 1 > 0 {
                flood_from_index(image, r, c - 1, old_color, new_color, visited);
            }
        }
    }

    let mut visited = HashSet::new();
    let from_color = image[sr as usize][sc as usize];
    flood_from_index(image, sr, sc, from_color, new_color, &mut visited);
    image.to_vec()
}

#[cfg(test)]
mod test {
    #[allow(unused)]
    use super::flood_fill;
    // every node gets updated
    // only first node gets update
    // node and some neighbors get updated
}
