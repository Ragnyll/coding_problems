// Flatland is a country with a number of cities, some of which have space stations. Cities are numbered consecutively and each has a road of length connecting it to the next city. It is not a circular route, so the first city doesn't connect with the last city. Determine the maximum distance from any city to its nearest space station.

#[allow(dead_code)]
fn flatland_space_stations(num_cities: u8, spacestations: &[u8]) -> u8 {
    // num space stations cannot be 0
    if spacestations.len() == 1 {
        // find distance to furthest index
        let dist_to_head = spacestations[0];
        let dist_to_tail = num_cities - (spacestations[0] + 1);

        if dist_to_head > dist_to_tail {
            return dist_to_head;
        }
        return dist_to_tail;
    }

    // find distance between each space station the one with the longest dist between will have the
    // furthest distance between it and a city
    let mut spacestations_distance: Vec<u8> = Vec::with_capacity(spacestations.len() - 1);

    // -2 because the second to last space station will calculate the dist to the last one
    let mut sorted_spacestations = spacestations.to_vec();
    sorted_spacestations.sort();

    for i in 0..spacestations.len() - 2 {
        spacestations_distance.push(spacestations[i + 1] - spacestations[i]);
    }

    let max_dist = spacestations_distance.iter().max().unwrap_or(&0);

    // check the max dist from the head or tail and take the greater of the 2
    // if thats greater than the dist between that and the max city distance return that
    let cities_before_first = sorted_spacestations[0];
    let cities_after_final = num_cities - (sorted_spacestations.len() as u8) - 1;
    let max_tail = vec![cities_after_final, cities_before_first]
        .iter()
        .max()
        .unwrap();
    if *max_tail > (max_dist / 2) + 1 {
        return *max_tail;
    }

    // if the dist is even the furthest city from a space station will be equidistant
    if max_dist % 2 == 0 {
        return max_dist / 2;
    }
    // if the dist is odd the furthest city from a space station will be unique
    max_dist / 2 + 1
}

#[cfg(test)]
mod test {
    use super::flatland_space_stations;
}
