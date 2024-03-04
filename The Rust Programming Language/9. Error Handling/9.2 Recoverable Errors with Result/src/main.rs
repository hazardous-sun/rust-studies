/*
Most errors aren’t serious enough to require the program to stop entirely. Sometimes, when a
function fails, it’s for a reason that you can easily interpret and respond to. For example, if you
try to open a file and that operation fails because the file doesn’t exist, you might want to create
the file instead of terminating the process.

Recall from “Handling Potential Failure with Result” in Chapter 2 that the Result enum is defined as
having two variants, Ok and Err, as follows:

enum Result<T, E> {
    Ok(T),
    Err(E),
}

The T and E are generic type parameters: we’ll discuss generics in more detail in Chapter 10. What
you need to know right now is that T represents the type of the value that will be returned in a
success case within the Ok variant, and E represents the type of the error that will be returned in
a failure case within the Err variant. Because Result has these generic type parameters, we can use
the Result type and the functions defined on it in many different situations where the successful
value and error value we want to return may differ.

Let’s call a function that returns a Result value because the function could fail. In Listing 9-3 we
try to open a file.
 */

use std::sync::atomic::spin_loop_hint;

fn using_result_for_error_handling() {
    use std::fs::File;

    let greeting_file_result = File::open("hello.txt");

    /*
    The return type of File::open is a Result<T, E>. The generic parameter T has been filled in by
    the implementation of File::open with the type of the success value, std::fs::File, which is a
    file handle. The type of E used in the error value is std::io::Error. This return type means the
    call to File::open might succeed and return a file handle that we can read from or write to. The
    function call also might fail: for example, the file might not exist, or we might not have
    permission to access the file. The File::open function needs to have a way to tell us whether it
    succeeded or failed and at the same time give us either the file handle or error information.
    This information is exactly what the Result enum conveys.

    In the case where File::open succeeds, the value in the variable greeting_file_result will be an
    instance of Ok that contains a file handle. In the case where it fails, the value in
    greeting_file_result will be an instance of Err that contains more information about the kind of
    error that happened.

    We need to add to the code in Listing 9-3 to take different actions depending on the value
    File::open returns. Listing 9-4 shows one way to handle the Result using a basic tool, the match
    expression that we discussed in Chapter 6.
     */

    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    /*
    Note that, like the Option enum, the Result enum and its variants have been brought into scope
    by the prelude, so we don’t need to specify Result:: before the Ok and Err variants in the match
    arms.

    When the result is Ok, this code will return the inner file value out of the Ok variant, and we
    then assign that file handle value to the variable greeting_file. After the match, we can use
    the file handle for reading or writing.

    The other arm of the match handles the case where we get an Err value from File::open. In this
    example, we’ve chosen to call the panic! macro. If there’s no file named hello.txt in our
    current directory and we run this code, we’ll see the following output from the panic! macro:

$ cargo run
   Compiling recoverable_erros v0.1.0 (path_to_project)
    Finished dev [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/project`
thread 'main' panicked at src/main.rs:55:23:
Problem opening the file: Os { code: 2, kind: NotFound, message: "No such file or directory" }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

    As usual, this output tells us exactly what has gone wrong
     */
}

fn matching_on_different_errors() {
    /*
    The code in Listing 9-4 will panic! no matter why File::open failed. However, we want to take
    different actions for different failure reasons: if File::open failed because the file doesn’t
    exist, we want to create the file and return the handle to the new file. If File::open failed
    for any other reason—for example, because we didn’t have permission to open the file—we still
    want the code to panic! in the same way as it did in Listing 9-4. For this we add an inner match
    expression, shown in Listing 9-5.
     */

    use std::fs::File;
    use std::io::ErrorKind;

    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
}

fn alternatives_to_using_match_with_result() {
    /*
    That’s a lot of match! The match expression is very useful but also very much a primitive. In
    Chapter 13, you’ll learn about closures, which are used with many of the methods defined on
    Result<T, E>. These methods can be more concise than using match when handling Result<T, E>
    values in your code.

    For example, here’s another way to write the same logic as shown in Listing 9-5, this time using
    closures and the unwrap_or_else method:
     */
    use std::fs::File;
    use std::io::ErrorKind;

    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    /*
    Although this code has the same behavior as Listing 9-5, it doesn’t contain any match
    expressions and is cleaner to read. Come back to this example after you’ve read Chapter 13, and
    look up the unwrap_or_else method in the standard library documentation. Many more of these
    methods can clean up huge nested match expressions when you’re dealing with errors.
     */
}

fn shortcuts_for_panic_on_error() {
    /*
    Using match works well enough, but it can be a bit verbose and doesn’t always communicate intent
    well. The Result<T, E> type has many helper methods defined on it to do various, more specific
    tasks. The unwrap method is a shortcut method implemented just like the match expression we
    wrote in Listing 9-4. If the Result value is the Ok variant, unwrap will return the value inside
    the Ok. If the Result is the Err variant, unwrap will call the panic! macro for us. Here is an
    example of unwrap in action:
     */

    use std::fs::File;
    // let greeting_file = File::open("hello.txt").unwrap();

    /*
    If we run this code without a hello.txt file, we’ll see an error message from the panic! call
    that the unwrap method makes:

    thread 'main' panicked at src/main.rs:156:49:
    called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }

    Similarly, the expect method lets us also choose the panic! error message. Using expect instead
    of unwrap and providing good error messages can convey your intent and make tracking down the
    source of a panic easier. The syntax of expect looks like this:
     */

    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");

    /*
    We use expect in the same way as unwrap: to return the file handle or call the panic! macro. The
    error message used by expect in its call to panic! will be the parameter that we pass to expect,
    rather than the default panic! message that unwrap uses. Here’s what it looks like:

    thread 'main' panicked at src/main.rs:171:10:
    hello.txt should be included in this project: Os { code: 2, kind: NotFound, message: "No such file or directory" }

    In production-quality code, most Rustaceans choose expect rather than unwrap and give more
    context about why the operation is expected to always succeed. That way, if your assumptions are
    ever proven wrong, you have more information to use in debugging.
     */
}

fn propagating_errors() {
    /*
    When a function’s implementation calls something that might fail, instead of handling the error
    within the function itself, you can return the error to the calling code so that it can decide
    what to do. This is known as propagating the error and gives more control to the calling code,
    where there might be more information or logic that dictates how the error should be handled
    than what you have available in the context of your code.

    For example, Listing 9-6 shows a function that reads a username from a file. If the file doesn’t
    exist or can’t be read, this function will return those errors to the code that called the
    function.
     */

    use std::fs::File;
    use std::io::{self, Read};

    fn read_username_from_file() -> Result<String, io::Error> {
        let username_file_result = File::open("hello.txt");

        let mut username_file = match username_file_result {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut username = String::new();

        match username_file.read_to_string(&mut username) {
            Ok(_) => Ok(username),
            Err(e) => Err(e),
        }
    }

    /*
    This function can be written in a much shorter way, but we’re going to start by doing a lot of
    it manually in order to explore error handling; at the end, we’ll show the shorter way. Let’s
    look at the return type of the function first: Result<String, io::Error>. This means the
    function is returning a value of the type Result<T, E> where the generic parameter T has been
    filled in with the concrete type String, and the generic type E has been filled in with the
    concrete type io::Error.

    If this function succeeds without any problems, the code that calls this function will receive
    an Ok value that holds a String—the username that this function read from the file. If this
    function encounters any problems, the calling code will receive an Err value that holds an
    instance of io::Error that contains more information about what the problems were. We chose
    io::Error as the return type of this function because that happens to be the type of the error
    value returned from both of the operations we’re calling in this function’s body that might
    fail: the File::open function and the read_to_string method.

    The body of the function starts by calling the File::open function. Then we handle the Result
    value with a match similar to the match in Listing 9-4. If File::open succeeds, the file handle
    in the pattern variable file becomes the value in the mutable variable username_file and the
    function continues. In the Err case, instead of calling panic!, we use the return keyword to
    return early out of the function entirely and pass the error value from File::open, now in the
    pattern variable e, back to the calling code as this function’s error value.

    So if we have a file handle in username_file, the function then creates a new String in variable
    username and calls the read_to_string method on the file handle in username_file to read the
    contents of the file into username. The read_to_string method also returns a Result because it
    might fail, even though File::open succeeded. So we need another match to handle that Result:
    if read_to_string succeeds, then our function has succeeded, and we return the username from the
    file that’s now in username wrapped in an Ok. If read_to_string fails, we return the error value
    in the same way that we returned the error value in the match that handled the return value of
    File::open. However, we don’t need to explicitly say return, because this is the last expression
    in the function.

    The code that calls this code will then handle getting either an Ok value that contains a
    username or an Err value that contains an io::Error. It’s up to the calling code to decide what
    to do with those values. If the calling code gets an Err value, it could call panic! and crash
    the program, use a default username, or look up the username from somewhere other than a file,
    for example. We don’t have enough information on what the calling code is actually trying to do,
    so we propagate all the success or error information upward for it to handle appropriately.

    This pattern of propagating errors is so common in Rust that Rust provides the question mark
    operator ? to make this easier.
     */
}

fn main() {
    shortcuts_for_panic_on_error();
}
