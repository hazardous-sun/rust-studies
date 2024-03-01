use std::collections::HashMap;

fn main() {
    exercise_1();
}

fn exercise_1() {
    /*
    Given a list of integers, use a vector and return the median (when sorted, the value in the
    middle position) and mode (the value that occurs most often; a hash map will be helpful here) of
    the list.
     */

    let mut values = vec![
        1, 2, 3, 4, 5, 6, 7, 5, 5, 5, 5, 5, 5, 5, 8, 9, 10, 1, 2, 4, 6, 3, 1, 4, 61, 74, 3,
    ];
    let mean = exercise_1_mean(&mut values);
    let median = exercise_1_median(&mut values);
    let mode = exercise_1_mode(&mut values);

    println!(
        "Exercise 1:\nValues: {:?}\nMean: {}\tMedian: {}\tMode: {:?}\n",
        values, mean, median, mode
    );
}

fn exercise_1_mean(values: &mut Vec<i32>) -> i32 {
    let values_len = values.len() as i32;
    let mut mean = 0;
    for value in values {
        mean += *value;
    }
    mean / values_len
}

fn exercise_1_median(values: &mut Vec<i32>) -> i32 {
    values.sort();
    values[values.len() / 2]
}

fn exercise_1_mode(values: &mut Vec<i32>) -> (i32, i32) {
    let mut map: HashMap<&i32, i32> = HashMap::new();
    for element in values {
        let count = map.entry(element).or_insert(0);
        *count += 1;
    }
    let mut mode: Option<(i32, i32)> = None;
    for (key, value) in map.iter() {
        match mode {
            Some((_, stored_value)) if value > &stored_value => {
                mode = Some((**key, *value));
            }
            Some(_) => {}
            None => mode = Some((**key, *value)),
        }
    }
    match mode {
        Some(_) => mode.unwrap(),
        None => (-1, -1),
    }
}
