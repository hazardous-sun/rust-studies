/*
Rust’s closures are anonymous functions you can save in a variable or pass as arguments to other
functions. You can create the closure in one place and then call the closure elsewhere to evaluate
it in a different context. Unlike functions, closures can capture values from the scope in which
they’re defined. We’ll demonstrate how these closure features allow for code reuse and behavior
customization.
 */

fn capturing_the_environment_with_closures() {
    /*
    We’ll first examine how we can use closures to capture values from the environment they’re
    defined in for later use. Here’s the scenario: Every so often, our t-shirt company gives away an
    exclusive, limited-edition shirt to someone on our mailing list as a promotion. People on the
    mailing list can optionally add their favorite color to their profile. If the person chosen for
    a free shirt has their favorite color set, they get that color shirt. If the person hasn’t
    specified a favorite color, they get whatever color the company currently has the most of.

    There are many ways to implement this. For this example, we’re going to use an enum called
    ShirtColor that has the variants Red and Blue (limiting the number of colors available for
    simplicity). We represent the company’s inventory with an Inventory struct that has a field
    named shirts that contains a Vec<ShirtColor> representing the shirt colors currently in stock.
    The method giveaway defined on Inventory gets the optional shirt color preference of the free
    shirt winner, and returns the shirt color the person will get. This setup is shown in Listing
    13-1:
     */

    #[derive(Debug, PartialEq, Copy, Clone)]
    enum ShirtColor {
        Red,
        Blue,
    }

    struct Inventory {
        shirts: Vec<ShirtColor>,
    }

    impl Inventory {
        fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
            user_preference.unwrap_or_else(|| self.most_stocked())
        }

        fn most_stocked(&self) -> ShirtColor {
            let mut num_red = 0;
            let mut num_blue = 0;

            for color in &self.shirts {
                match color {
                    ShirtColor::Red => num_red += 1,
                    ShirtColor::Blue => num_blue += 1,
                }
            }
            if num_red > num_blue {
                ShirtColor::Red
            } else {
                ShirtColor::Blue
            }
        }
    }

    fn caller() {
        let store = Inventory {
            shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
        };

        let user_pref1 = Some(ShirtColor::Red);
        let giveaway1 = store.giveaway(user_pref1);
        println!(
            "The user with preference {:?} gets {:?}",
            user_pref1, giveaway1
        );

        let user_pref2 = None;
        let giveaway2 = store.giveaway(user_pref2);
        println!(
            "The user with preference {:?} gets {:?}",
            user_pref2, giveaway2
        );
    }

    /*
    The store defined in main has two blue shirts and one red shirt remaining to distribute for this
    limited-edition promotion. We call the giveaway method for a user with a preference for a red
    shirt and a user without any preference.

    Again, this code could be implemented in many ways, and here, to focus on closures, we’ve stuck
    to concepts you’ve already learned except for the body of the giveaway method that uses a
    closure. In the giveaway method, we get the user preference as a parameter of type
    Option<ShirtColor> and call the unwrap_or_else method on user_preference. The unwrap_or_else
    method on Option<T> is defined by the standard library. It takes one argument: a closure without
    any arguments that returns a value T (the same type stored in the Some variant of the Option<T>,
    in this case ShirtColor). If the Option<T> is the Some variant, unwrap_or_else returns the value
    from within the Some. If the Option<T> is the None variant, unwrap_or_else calls the closure and
    returns the value returned by the closure.

    We specify the closure expression || self.most_stocked() as the argument to unwrap_or_else. This
    is a closure that takes no parameters itself (if the closure had parameters, they would appear
    between the two vertical bars). The body of the closure calls self.most_stocked(). We’re
    defining the closure here, and the implementation of unwrap_or_else will evaluate the closure
    later if the result is needed.

    Running this code prints:

    $ cargo run
       Compiling shirt-company v0.1.0 (file:///projects/shirt-company)
        Finished dev [unoptimized + debuginfo] target(s) in 0.27s
         Running `target/debug/shirt-company`
    The user with preference Some(Red) gets Red
    The user with preference None gets Blue

    One interesting aspect here is that we’ve passed a closure that calls self.most_stocked() on the
    current Inventory instance. The standard library didn’t need to know anything about the
    Inventory or ShirtColor types we defined, or the logic we want to use in this scenario. The
    closure captures an immutable reference to the self Inventory instance and passes it with the
    code we specify to the unwrap_or_else method. Functions, on the other hand, are not able to
    capture their environment in this way.
     */
}

fn closure_type_inference_and_annotation() {
    /*
    There are more differences between functions and closures. Closures don’t usually require you to
    annotate the types of the parameters or the return value like fn functions do. Type annotations
    are required on functions because the types are part of an explicit interface exposed to your
    users. Defining this interface rigidly is important for ensuring that everyone agrees on what
    types of values a function uses and returns. Closures, on the other hand, aren’t used in an
    exposed interface like this: they’re stored in variables and used without naming them and
    exposing them to users of our library.

    Closures are typically short and relevant only within a narrow context rather than in any
    arbitrary scenario. Within these limited contexts, the compiler can infer the types of the
    parameters and the return type, similar to how it’s able to infer the types of most variables
    (there are rare cases where the compiler needs closure type annotations too).

    As with variables, we can add type annotations if we want to increase explicitness and clarity
    at the cost of being more verbose than is strictly necessary. Annotating the types for a closure
    would look like the definition shown in Listing 13-2. In this example, we’re defining a closure
    and storing it in a variable rather than defining the closure in the spot we pass it as an
    argument as we did in Listing 13-1.
     */

    use std::thread;
    use std::time::Duration;

    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    /*
    With type annotations added, the syntax of closures looks more similar to the syntax of
    functions. Here we define a function that adds 1 to its parameter and a closure that has the
    same behavior, for comparison. We’ve added some spaces to line up the relevant parts. This
    illustrates how closure syntax is similar to function syntax except for the use of pipes and the
    amount of syntax that is optional:
     */


    /*
    fn add_one_v1(x: u32) -> u32 { x + 1 }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x| { x + 1 };
    let add_one_v4 = |x| x + 1;
    */

    /*
    The first line shows a function definition, and the second line shows a fully annotated closure
    definition. In the third line, we remove the type annotations from the closure definition. In
    the fourth line, we remove the brackets, which are optional because the closure body has only
    one expression. These are all valid definitions that will produce the same behavior when they’re
    called. The add_one_v3 and add_one_v4 lines require the closures to be evaluated to be able to
    compile because the types will be inferred from their usage. This is similar to
    let v = Vec::new(); needing either type annotations or values of some type to be inserted into
    the Vec for Rust to be able to infer the type.

    For closure definitions, the compiler will infer one concrete type for each of their parameters
    and for their return value. For instance, Listing 13-3 shows the definition of a short closure
    that just returns the value it receives as a parameter. This closure isn’t very useful except
    for the purposes of this example. Note that we haven’t added any type annotations to the
    definition. Because there are no type annotations, we can call the closure with any type, which
    we’ve done here with String the first time. If we then try to call example_closure with an
    integer, we’ll get an error.


    let example_closure = |x| x;

    let s = example_closure(String::from("hello"));
    let n = example_closure(5);


    The compile gives us this error:

    $ cargo run
       Compiling closure-example v0.1.0 (file:///projects/closure-example)
    error[E0308]: mismatched types
     --> src/main.rs:5:29
      |
    5 |     let n = example_closure(5);
      |             --------------- ^- help: try using a conversion method: `.to_string()`
      |             |               |
      |             |               expected struct `String`, found integer
      |             arguments to this function are incorrect
      |
    note: closure parameter defined here
     --> src/main.rs:2:28
      |
    2 |     let example_closure = |x| x;
      |                            ^

    For more information about this error, try `rustc --explain E0308`.
    error: could not compile `closure-example` due to previous error

    The first time we call example_closure with the String value, the compiler infers the type of x
    and the return type of the closure to be String. Those types are then locked into the closure in
    example_closure, and we get a type error when we next try to use a different type with the same
    closure.
     */
}

fn capturing_references_or_moving_ownership() {
    /*
    Closures can capture values from their environment in three ways, which directly map to the
    three ways a function can take a parameter: borrowing immutably, borrowing mutably, and taking
    ownership. The closure will decide which of these to use based on what the body of the function
    does with the captured values.

    In Listing 13-4, we define a closure that captures an immutable reference to the vector named
    list because it only needs an immutable reference to print the value:
     */

    {
        let list = vec![1, 2, 3];
        println!("Before defining closure: {:?}", list);

        let only_borrows = || println!("From closure: {:?}", list);

        println!("Before calling closure: {:?}", list);
        only_borrows();
        println!("After calling closure: {:?}", list);
    }

    /*
    This example also illustrates that a variable can bind to a closure definition, and we can later
    call the closure by using the variable name and parentheses as if the variable name were a
    function name.

    Because we can have multiple immutable references to list at the same time, list is still
    accessible from the code before the closure definition, after the closure definition but before
    the closure is called, and after the closure is called. This code compiles, runs, and prints:


    $ cargo run
       Compiling closure-example v0.1.0 (file:///projects/closure-example)
        Finished dev [unoptimized + debuginfo] target(s) in 0.43s
         Running `target/debug/closure-example`
    Before defining closure: [1, 2, 3]
    Before calling closure: [1, 2, 3]
    From closure: [1, 2, 3]
    After calling closure: [1, 2, 3]


    Next, in Listing 13-5, we change the closure body so that it adds an element to the list vector.
    The closure now captures a mutable reference:
     */

    {
        let mut list = vec![1, 2, 3];
        println!("Before defining closure: {:?}", list);

        let mut borrows_mutably = || list.push(7);

        borrows_mutably();
        println!("After calling closure: {:?}", list);
    }

    /*
    This code compiles, runs, and prints:


    $ cargo run
       Compiling closure-example v0.1.0 (file:///projects/closure-example)
        Finished dev [unoptimized + debuginfo] target(s) in 0.43s
         Running `target/debug/closure-example`
    Before defining closure: [1, 2, 3]
    After calling closure: [1, 2, 3, 7]


    Note that there’s no longer a println! between the definition and the call of the
    borrows_mutably closure: when borrows_mutably is defined, it captures a mutable reference to
    list. We don’t use the closure again after the closure is called, so the mutable borrow ends.
    Between the closure definition and the closure call, an immutable borrow to print isn’t allowed
    because no other borrows are allowed when there’s a mutable borrow. Try adding a println! there
    to see what error message you get!

    If you want to force the closure to take ownership of the values it uses in the environment even
    though the body of the closure doesn’t strictly need ownership, you can use the move keyword
    before the parameter list.

    This technique is mostly useful when passing a closure to a new thread to move the data so that
    it’s owned by the new thread. We’ll discuss threads and why you would want to use them in detail
    in Chapter 16 when we talk about concurrency, but for now, let’s briefly explore spawning a new
    thread using a closure that needs the move keyword. Listing 13-6 shows Listing 13-4 modified to
    print the vector in a new thread rather than in the main thread:
     */

    {
        use std::thread;

        let list = vec![1, 2, 3];
        println!("Before defining closure: {:?}", list);

        thread::spawn(move || println!("From thread: {:?}", list))
            .join()
            .unwrap();
    }

    /*
    We spawn a new thread, giving the thread a closure to run as an argument. The closure body
    prints out the list. In Listing 13-4, the closure only captured list using an immutable
    reference because that's the least amount of access to list needed to print it. In this example,
    even though the closure body still only needs an immutable reference, we need to specify that
    list should be moved into the closure by putting the move keyword at the beginning of the
    closure definition. The new thread might finish before the rest of the main thread finishes, or
    the main thread might finish first. If the main thread maintained ownership of list but ended
    before the new thread did and dropped list, the immutable reference in the thread would be
    invalid. Therefore, the compiler requires that list be moved into the closure given to the new
    thread so the reference will be valid. Try removing the move keyword or using list in the main
    thread after the closure is defined to see what compiler errors you get!
     */
}

fn moving_captured_values_out_of_closures_and_the_fn_traits() {
    /*
    Once a closure has captured a reference or captured ownership of a value from the environment
    where the closure is defined (thus affecting what, if anything, is moved into the closure), the
    code in the body of the closure defines what happens to the references or values when the
    closure is evaluated later (thus affecting what, if anything, is moved out of the closure). A
    closure body can do any of the following: move a captured value out of the closure, mutate the
    captured value, neither move nor mutate the value, or capture nothing from the environment to
    begin with.

    The way a closure captures and handles values from the environment affects which traits the
    closure implements, and traits are how functions and structs can specify what kinds of closures
    they can use. Closures will automatically implement one, two, or all three of these Fn traits,
    in an additive fashion, depending on how the closure’s body handles the values:

    1. FnOnce applies to closures that can be called once. All closures implement at least this
    trait, because all closures can be called. A closure that moves captured values out of its body
    will only implement FnOnce and none of the other Fn traits, because it can only be called once.
    2. FnMut applies to closures that don’t move captured values out of their body, but that might
    mutate the captured values. These closures can be called more than once.
    3. Fn applies to closures that don’t move captured values out of their body and that don’t
    mutate captured values, as well as closures that capture nothing from their environment. These
    closures can be called more than once without mutating their environment, which is important in
    cases such as calling a closure multiple times concurrently.

    Let’s look at the definition of the unwrap_or_else method on Option<T> that we used in Listing
    13-1:


    impl<T> Option<T> {
        pub fn unwrap_or_else<F>(self, f: F) -> T
        where
            F: FnOnce() -> T
        {
            match self {
                Some(x) => x,
                None => f(),
            }
        }
    }


    Note: Functions can implement all three of the Fn traits too. If what we want to do doesn’t
    require capturing a value from the environment, we can use the name of a function rather than a
    closure where we need something that implements one of the Fn traits. For example, on an
    Option<Vec<T>> value, we could call unwrap_or_else(Vec::new) to get a new, empty vector if the
    value is None.

    Now let’s look at the standard library method sort_by_key defined on slices, to see how that
    differs from unwrap_or_else and why sort_by_key uses FnMut instead of FnOnce for the trait
    bound. The closure gets one argument in the form of a reference to the current item in the slice
    being considered, and returns a value of type K that can be ordered. This function is useful
    when you want to sort a slice by a particular attribute of each item. In Listing 13-7, we have a
    list of Rectangle instances and we use sort_by_key to order them by their width attribute from
    low to high:
     */

    {
        #[derive(Debug)]
        struct Rectangle {
            width: u32,
            height: u32,
        }

        fn caller() {
            let mut list = [
                Rectangle { width: 10, height: 1 },
                Rectangle { width: 3, height: 5 },
                Rectangle { width: 7, height: 12 },
            ];

            list.sort_by_key(|r| r.width);
            println!("{:#?}", list);
        }
    }

    /*
    This code prints:


    $ cargo run
       Compiling rectangles v0.1.0 (file:///projects/rectangles)
        Finished dev [unoptimized + debuginfo] target(s) in 0.41s
         Running `target/debug/rectangles`
    [
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
        Rectangle {
            width: 10,
            height: 1,
        },
    ]


    The reason sort_by_key is defined to take an FnMut closure is that it calls the closure multiple
    times: once for each item in the slice. The closure |r| r.width doesn’t capture, mutate, or move
    out anything from its environment, so it meets the trait bound requirements.

    In contrast, Listing 13-8 shows an example of a closure that implements just the FnOnce trait,
    because it moves a value out of the environment. The compiler won’t let us use this closure with
    sort_by_key:


    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    fn main() {
        let mut list = [
            Rectangle { width: 10, height: 1 },
            Rectangle { width: 3, height: 5 },
            Rectangle { width: 7, height: 12 },
        ];

        let mut sort_operations = vec![];
        let value = String::from("by key called");

        list.sort_by_key(|r| {
            sort_operations.push(value);
            r.width
        });
        println!("{:#?}", list);
    }


    This is a contrived, convoluted way (that doesn’t work) to try and count the number of times
    sort_by_key gets called when sorting list. This code attempts to do this counting by pushing
    value—a String from the closure’s environment—into the sort_operations vector. The closure
    captures value then moves value out of the closure by transferring ownership of value to the
    sort_operations vector. This closure can be called once; trying to call it a second time
    wouldn’t work because value would no longer be in the environment to be pushed into
    sort_operations again! Therefore, this closure only implements FnOnce. When we try to compile
    this code, we get this error that value can’t be moved out of the closure because the closure
    must implement FnMut:


    $ cargo run
       Compiling rectangles v0.1.0 (file:///projects/rectangles)
    error[E0507]: cannot move out of `value`, a captured variable in an `FnMut` closure
      --> src/main.rs:18:30
       |
    15 |     let value = String::from("by key called");
       |         ----- captured outer variable
    16 |
    17 |     list.sort_by_key(|r| {
       |                      --- captured by this `FnMut` closure
    18 |         sort_operations.push(value);
       |                              ^^^^^ move occurs because `value` has type `String`, which does not implement the `Copy` trait

    For more information about this error, try `rustc --explain E0507`.
    error: could not compile `rectangles` due to previous error


    The error points to the line in the closure body that moves value out of the environment. To fix
    this, we need to change the closure body so that it doesn’t move values out of the environment.
    To count the number of times sort_by_key is called, keeping a counter in the environment and
    incrementing its value in the closure body is a more straightforward way to calculate that. The
    closure in Listing 13-9 works with sort_by_key because it is only capturing a mutable reference
    to the num_sort_operations counter and can therefore be called more than once:
     */

    {
        #[derive(Debug)]
        struct Rectangle {
            width: u32,
            height: u32,
        }

        fn caller() {
            let mut list = [
                Rectangle { width: 10, height: 1 },
                Rectangle { width: 3, height: 5 },
                Rectangle { width: 7, height: 12 },
            ];

            let mut num_sort_operations = 0;
            list.sort_by_key(|r| {
                num_sort_operations += 1;
                r.width
            });
            println!("{:#?}, sorted in {num_sort_operations} operations", list);
        }
    }

    /*
    The Fn traits are important when defining or using functions or types that make use of closures.
    In the next section, we’ll discuss iterators. Many iterator methods take closure arguments, so
    keep these closure details in mind as we continue!
     */
}

fn main() {
    capturing_references_or_moving_ownership();
}
