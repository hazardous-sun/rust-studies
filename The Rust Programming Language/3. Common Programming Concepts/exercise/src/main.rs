/*
You made it! This was a sizable chapter: you learned about variables, scalar and compound data
types, functions, comments, if expressions, and loops! To practice with the concepts discussed in
this chapter, try building programs to do the following:

1. Convert temperatures between Fahrenheit and Celsius.
2. Generate the nth Fibonacci number.
*/

// 1
fn fahrenheit_to_celsius(temperature: f64) -> f64 {
    ((temperature - 32.0) * 5.0) / 9.0
}

fn celsius_to_fahrenheit(temperature: f64) -> f64 {
    (temperature * (9.0/5.0)) + 32.0
}

// 2
fn iterative_fibonacci(pos: i32) -> i32 {
    // 0 1 2 3 4 5 6  7  8  9
    // 1 1 2 3 5 8 13 21 34 55
    if pos == 1 || pos == 2 {
        return 1;
    }

    let mut val1 = 1;
    let mut val2 = 2;

    for _ in 2..pos {
        let temp = val2;
        val2 += val1;
        val1 = temp;
    }
    val2
}

fn recursive_fibonacci(n: i32) -> i32 {
    match n {
        0 => 1,
        1 => 1,
        _ => recursive_fibonacci(n - 1) + recursive_fibonacci(n - 2)
    }
}

fn main() {
    let x = celsius_to_fahrenheit(32.0);
    println!("x = {x}");
}
