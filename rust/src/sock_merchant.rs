use std::collections::HashMap;
// https://www.hackerrank.com/challenges/sock-merchant/problem?h_l=interview&playlist_slugs%5B%5D=interview-preparation-kit&playlist_slugs%5B%5D=warmup

#[allow(dead_code)]
fn create_sock_map(socks: Vec<&str>) -> HashMap<&str, usize> {
    let mut socktype_to_num_sock: HashMap<&str, usize> = HashMap::new();

    for sock in socks {
        socktype_to_num_sock.insert(sock, socktype_to_num_sock.get(sock).unwrap_or(&0) + 1);
    }

    socktype_to_num_sock
}

#[allow(dead_code)]
fn pairs(sorted_socks: HashMap<&str, usize>) -> usize {
    let mut num_pairs = 0;
    for value in sorted_socks.values() {
        num_pairs += value / 2;
    }
    num_pairs
}
