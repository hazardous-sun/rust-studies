use std::collections::HashMap;
use std::io;

fn main() {
    print_separator();
    exercise_1();
    print_separator();
    exercise_2();
    print_separator();
    exercise_3();
}

// ********************************************************
fn print_separator() {
    println!();
    let mut i = 0;
    while i < 30 {
        print!("*");
        i += 1;
    }
    println!();
}

// ********************************************************
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
        "Exercise 1:\nValues: {:?}\nMean: {}\tMedian: {}\tMode: {:?}",
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

// ********************************************************
fn exercise_2() {
    /*
    Convert strings to pig latin. The first consonant of each word is moved to the end of the word
    and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added
    to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!
     */

    let original_string = String::from("Mary had a little lamb");
    let new_string = exercise_2_convert_string(&original_string);
    println!("Original string: {original_string}\nNew string: {new_string}")
}

fn exercise_2_convert_string(original_string: &String) -> String {
    let vowels = vec!['a', 'e', 'i', 'o', 'u'];
    let mut returned_string = String::new();
    'first_loop: for word in original_string.split_whitespace() {
        if let Some(first_char) = word.chars().next() {
            if vowels.contains(&first_char) {
                returned_string.push_str(&format!("{word}-hay "));
                continue 'first_loop;
            }
        }
        returned_string.push_str(&format!("{}-fay ", &word[0..]));
    }
    returned_string.trim_end().to_string()
}

// ********************************************************
fn exercise_3() {
    /*
    Using a hash map and vectors, create a text interface to allow a user to add employee names to a 
    department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then 
    let the user retrieve a list of all people in a department or all people in the company by 
    department, sorted alphabetically.
     */

    let mut company: HashMap<String, Vec<_>> = HashMap::new();
    let mut input = String::new();

    println!("\
    1. List all people in company\n\
    2. List departments\n\
    3. List all people in a department\n\
    4. Add department\n\
    5. Add employee to department\n");

    loop {
        get_user_input(&mut input);
        let choice: i32 = match input.trim().parse() {
            Ok(value) => value,
            Err(_) => continue
        };
        match choice {
            1 => exercise_3_list_people_company(&mut company),
            2 => exercise_3_list_departments(&mut company),
            3 => exercise_3_list_people_department(&mut company),
            4 => exercise_3_add_department(&mut company),
            5 => exercise_3_add_people_department(&mut company),
            _ => continue
        }
    }
}

fn get_user_input(input: &mut String) {
    io::stdin()
        .read_line(input)
        .expect("Failed to read user input");
}

fn exercise_3_list_people_company(company: &mut HashMap<String, Vec<String>>) {}

fn exercise_3_list_departments(company: &mut HashMap<String, Vec<String>>) {}

fn exercise_3_list_people_department(company: &mut HashMap<String, Vec<String>>) {}

fn exercise_3_add_department(company: &mut HashMap<String, Vec<String>>) {
    let mut department = String::new();
    get_user_input(&mut department);
    company.entry(department.clone()).or_else(Vec::new());
}

fn exercise_3_add_people_department(company: &mut HashMap<String, Vec<String>>) {}
