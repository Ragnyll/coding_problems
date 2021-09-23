// https://leetcode.com/problems/container-with-most-water/
// Given n non-negative integers a1, a2, ..., an , where each represents a point at coordinate (i,
// ai). n vertical lines are drawn such that the two endpoints of the line i is at (i, ai) and (i,
// 0). Find two lines, which, together with the x-axis forms a container, such that the container
// contains the most water.
use std::cmp;

#[allow(dead_code)]
fn most_water(bars: &[usize]) -> usize {
    let mut left_index = 0;
    let mut right_index = bars.len() - 1;

    let get_area_between_bars = |left: usize, right: usize, bars: &[usize]| {
        cmp::min(bars[left], bars[right]) * (right - left)
    };

    // pick lesser of two heights and multiply by distance between the two
    let mut max_area = get_area_between_bars(left_index, right_index, bars);

    while left_index != right_index {
        match bars[left_index] > bars[right_index] {
            true => right_index -= 1,
            false => left_index += 1,
        }
        let area = get_area_between_bars(left_index, right_index, bars);
        if area > max_area {
            max_area = area;
        }

    }
    max_area
}

#[cfg(test)]
pub mod test {
    use super::most_water;

    #[test]
    fn ex_1() {
        assert_eq!(1, most_water(&[1,1]));
    }

    #[test]
    fn ex_2() {
        assert_eq!(16, most_water(&[4, 3, 2, 1, 4]));
    }

    #[test]
    fn ex_3() {
        assert_eq!(2, most_water(&[1, 2, 1]));
    }

    #[test]
    fn ex_4() {
        assert_eq!(6, most_water(&[ 1, 5, 4, 3 ]));
    }

    #[test]
    fn ex_5() {
        assert_eq!(12, most_water(&[ 3, 1, 2, 4, 5 ]));
    }
}
