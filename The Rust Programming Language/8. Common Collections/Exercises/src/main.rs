use std::collections::HashMap;

fn exercise_1() {
    /*
    Given a list of integers, use a vector and return the median (when sorted, the value in the
    middle position) and mode (the value that occurs most often; a hash map will be helpful here) of
    the list.
     */

    let values = vec![1,2,3,4,5,6,7,8,9,10,6,3,1,4,61,74,3];

    let mut map = HashMap::new();

    for element in &values {
        let count = map.entry(element).or_insert(0);
        *count += 1;
    }

    let median = values[values.len() / 2];
    let mut mode = (0, 0);

    for (&key, &value) in map.iter().collect::<Vec<_>>() {
        if value > *mode.1 {
            mode = (*key, *value);
        }
    }

    println!("median: {} | mode: {:?}", median, mode);
}

fn main() {
    exercise_1();
}
