//  There is a new mobile game that starts with consecutively numbered clouds. Some of the clouds are thunderheads and others are cumulus. The player can jump on any cumulus cloud having a number that is equal to the number of the current cloud plus or
//
//  . The player must avoid the thunderheads. Determine the minimum number of jumps it will take to jump from the starting postion to the last cloud. It is always possible to win the game.
//
//  For each game, you will get an array of clouds numbered
//  if they are safe or if they must be avoided.

const SAFE_CLOUD: u8 = 0;

#[allow(dead_code)]
fn jump(clouds: &[u16]) -> u16 {
    if clouds.len() <= 1 {
        return 0;
    }

    let mut current_cloud = 0;
    let mut num_jumps = 0;

    while current_cloud != clouds.len() - 1 {
        if current_cloud + 2 <= clouds.len() - 1 && clouds[current_cloud + 2] == SAFE_CLOUD as u16 {
            current_cloud += 2;
        } else {
            current_cloud += 1;
        }

        num_jumps += 1;
    }

    num_jumps
}

#[cfg(test)]
mod test {
    use super::jump;

    #[test]
    fn test_jump() {
        assert_eq!(jump(&[0, 0, 0, 0, 1, 0]), 3);
        assert_eq!(jump(&[0, 0, 1, 0, 0, 1, 0]), 4);
        assert_eq!(jump(&[0, 0, 0, 1, 0, 0]), 3)
    }
}
