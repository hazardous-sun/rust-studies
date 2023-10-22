/*
Slices let you reference a contiguous sequence of elements in a collection rather than the whole
collection. A slice is a kind of reference, so it does not have ownership.

Here's a small programming problem: write a function that takes a string of words separated by
spaces and returns the first word it finds in that string. If the function doesn't find a space in
the string, the whole string must be one word, so the entire string should be returned.

Let's work through how we'd write the signature of this function without using slices, to understand
the problem that slices will solve:

fn first_word(s: &String) -> ?

The first_word function has a &String as a parameter. We don't want ownership, so this is fine. But
what should we return? We don't really have a way to talk about part of a string. However, we could
return the index of the end of the word, indicated by a space.

 */

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

/*
Because we need to go through the String element by element and check whether a value is a space,
we'll convert our String to an array of bytes using the "as_bytes" method.

"let bytes = s.as_bytes();"

Next, we create an iterator over the array of bytes using the "iter" method:

"for (i, &item) in bytes.iter().enumerate() {"

We'll discuss iterators in more detail in Chapter 13. For now, know that "iter" is a method that
returns each element in a collection and that "enumerate" wraps the result of "iter" and returns
each elements as part of a tuple instead. The first element of the tuple returned from "enumerate"
is the index, and the second element is a reference to the element. This is a bit more convenient
than calculating the index ourselves.

Because the "enumerate" method returns a tuple, we can use patterns to destructure that tuple. We'll
be discussing patterns more in Chapter 6. In the "for" loop, we specify a pattern that has "i" for
the index in the tuple and "&item" for the single byte in the tuple. Because we get a reference to
the element from ".iter().enumerate" we use "&" in the pattern.

Inside the "for" loop, we search for the byte that represents the space by using the byte literal
syntax. If we find a space, we return the position. Otherwise, we return the length of the string by
using "s.len()".

We now have a way to find out the index of the end of the first word in the string, but there's a
problem. We're returning a "usize" on its own, but it's only a meaningful number in the context of
the &String. In other words, because it's a separate value from the String, there's no guarantee
that it will still be valid in the future. Consider the following program that uses the "first_word"
function:
 */

fn crashing_previous_function() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5

    s.clear(); // this empties the String, making it equal to ""

    // "word" still has the value 5 here, but there's no more string that we could meaningfully use
    // the value 5 with. "word" is now totally invalid!
}

/*
This program compiles without any error and would also do so if we used "word" after calling
"s.clear()". Because "word" isn't connected to the state of "s" at all, "word" still contains the
value 5. We could use that value 5 with the variable "s" to try to extract the first word out, but
this would be a bug because the contents of "s" have changed since we saved 5 in "word".

Having to worry about the index in "word" getting out of sync with the data in "s" is tedious and
error prone! Managing theses indices is even more brittle if we write a "second_word" function. Its
signature would have to look like this:

"fn second_word(s: &String) -> (usize, usize) {"

Now we're tracking a starting AND an ending index, and we have even more values that were calculated
from data in a particular state but aren't tied to that state at all. We have three unrelated
variables floating around that need to be kept in sync.

Luckily, Rust has a solution to this problem: string slices.

A "string slice" is a reference to part of a String, and it looks like this:
 */

fn string_slicing() {
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..1];
}

/*
Rather than a reference to the entire String, "hello" is a reference to a portion of the String,
specified in the extra "[0..5]" bit. We create slices using a range within brackets by specifying
"[starting_index..ending_index]", where "starting_index" is the first position in the slice and
"ending_index" is one more than the last position in the slice. Internally, the slice data structure
stores the starting position and the length of the slice, which corresponds to "ending_index" minus
"starting_index". So, in the case of "let world = &s[6..11];", the "world" would be a slice that
contains a pointer to the byte at index 6 of "s" with a length of 5.

--------------------        -----------------
| name     | value |        | index | value |
|-------------------        |-------|-------|
| ptr      |       | -----> | 0     | h     |
| len      | 11    |        | 1     | e     |
| capacity | 11    |        | 2     | l     |
--------------------        | 3     | l     |
| name | value |            | 4     | o     |
|--------------|            | 5     |       |
| ptr  |       | ---------> | 6     | w     |
| len  | 5     |            | 7     | o     |
----------------            | 8     | r     |
                            | 9     | l     |
                            | 10    | d     |
                            -----------------
 */

fn equality_between_ranges() {
    /*
    With Rust's ".." range syntax, if you want to start at index 0, you can drop the value before the
    two periods. In other others, these are equal:
     */

    let s = String::from("hello");

    let slice = &s[0..2]; // slice = "he"
    println!("{}", slice);

    let slice = &s[..2];  // slice = "he"
    println!("{}", slice);

    /*
    By the same toke, if you slice includes the last byte of the String, you can drop the trailing
    number. That means these are equal:
     */

    let s = String::from("hello");
    let len = s.len();

    let slice = &s[3..len]; // slice = lo
    println!("{}", slice);

    let slice = &s[3..]; // slice = lo
    println!("{}", slice);

    /*
    You can also drop both values to take a slice of the entire string. So these are equal:
     */

    let s = String::from("hello");
    let len = s.len();

    let slice = &s[0..len]; // slice = "hello"
    println!("{}", slice);

    let slice = &s[..]; // slice = "hello"
    println!("{}", slice);

    /*
    NOTE: String slice range indices must occur at valid UTF-8 character boundaries. If you attempt
    to create a string slice in the middle of a multibyte character, your program will exit with an
    error. For the purposes of introducing string slices, we are assuming ASCII only in this
    section; a more thorough discussion of UTF-8 handling is in the "Storing UFT-8 Encoded Text with
    String" section of Chapter 8.
     */
}

/*
With all this information in mind, let's rewrite "first_word" to return a slice. The type that
signifies "string slice" is written as "&str":
 */

fn renewed_first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..] // returns the whole string, because there is only one word if there are no spaces
}

/*
We get the index for the end of the word the same way we did before, by looking for the first
occurrence of a space. When we find a space, we return a string slice using the start of the string
and the index of the space as the starting and ending indices.

Now when we call "renewed_first_word", we get back a single value that is tied to the underlying
data. The value is made up of a reference to the starting point of the slice and the number of
elements in the slice.

Returning a slice would also work for a "second_word" function

"fn second_word(s: &String) -> &str {"

We now have a straightforward API that's much harder to mess up because the compiler will ensure the
reference into the String remain valid. Remember the bug in crashing_previous_function, when we got
the index to the end of the fist word but then cleared the string so our index was invalid? That
code was logically incorrect but didn't show any immediate errors. The problems would show up later
if we kept trying to use the first word index with an emptied string. Slices make this bug
impossible and let us know we have a problem with our code much sooner. Using the slice version of
"first_word" will throw a compile-time error:

fn crashing_slice_function() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear(); // error!

    println!("the first word is: {}", word);
}

"crashing_slice_function" will raise error message "error[E0502]: cannot borrow `s` as mutable
because it is also borrowed as immutable"

Recall from the borrowing rules that if we have an immutable reference to something, we cannot also
take a mutable reference. Because "clear" needs to truncate the String, it needs to get a mutable
reference. The "println!" after the call to "clear" uses the reference in "word", so the immutable
reference must still be active at that point. Rust disallows the mutable reference in "clear" and
the immutable reference in "word" from existing at the same time, and compilation fails. Not only
has Rust made our API easier to use, but it has also eliminated an entire class of error at compile
time!

Knowing that you can take slices of literals and String values leads us to one more improvement on
first_word, and that's its signature:

"fn first_word (s : &String) -> &str {"

A more experienced Rustacean would write the following signature instead because it allows us to use
the same function on both &String values and &str values:

"fn firts_word(s: &str) -> &str {"

If we have a string slice, we can pass a slice of the String or a reference to the String. This
flexibility takes advantages of "deref coercions", a feature we will cover in "Implicit Deref
Coercions with Functions and Methods" section of Chapter 15.

Defining a function to take a string slice instead of a reference to a String makes out API more
general and useful without losing any functionality:
 */

fn final_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn testing_final_first_word() {
    let my_string = String::from("hello word this is me");

    // "final_first_word" works on slices of "String"s, whether partial or whole
    let word = final_first_word(&my_string[0..6]);
    let word = final_first_word(&my_string[..]);

    // "final_first_word" also works on references to "String"s, which are equivalent to whole
    // slices of "String"s
    let word = final_first_word(&my_string);

    let my_string_literal = "hello world";

    // "final_first_word" works on slices of string literals, whether partial or whole
    let word = final_first_word(&my_string_literal[0..6]);
    let word = final_first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already, this works too, without the slice syntax
    let word = final_first_word(my_string_literal);
}

fn other_slices() {
    /*
    String slices, as you might imagine, are specific to strings. But there's a more general slice
    type too. Consider this array:
     */

    let a = [1, 2, 3, 4, 5];

    /*
    Just as we might want to refer to part of a string, we might want tot refer to part of an array.
    We'd do so like this:
     */

    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);

    /*
    This slice has the type &[i32]. It works the same way as string slices do, by storing a
    reference to the first element and a length. You'll use this kind of slice for all sorts of
    other collections. We'll discuss these collections in detail when we talk about vectors in
    Chapter 8.
     */
}

/*
Summary

The concepts of ownership, borrowing and slices ensure memory safety in Rust programs at compile
time. The Rust language gives you control over your memory usage in the same way as other systems
programming languages, but having the owner of data automatically clean up that data when the owner
goes out of scope means you don't have to write and debug extra code to get this control.

Ownership affects how lots of other parts of Rust work, so we'll talk about these concepts further
throughout the rest of the book. Let's move on to Chapter 5 and look at grouping pieces of data
together in a "struct".
 */

fn main() {
    println!("hello");
}
