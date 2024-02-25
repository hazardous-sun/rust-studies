fn what_is_a_string() {
    /*
    We’ll first define what we mean by the term string. Rust has only one string type in the core
    language, which is the string slice str that is usually seen in its borrowed form &str. In
    Chapter 4, we talked about string slices, which are references to some UTF-8 encoded string data
    stored elsewhere. String literals, for example, are stored in the program’s binary and are
    therefore string slices.

    The String type, which is provided by Rust’s standard library rather than coded into the core
    language, is a growable, mutable, owned, UTF-8 encoded string type. When Rustaceans refer to
    “strings” in Rust, they might be referring to either the String or the string slice &str types,
    not just one of those types. Although this section is largely about String, both types are used
    heavily in Rust’s standard library, and both String and string slices are UTF-8 encoded.
     */
}

fn creating_a_new_string() {
    /*
    Many of the same operations available with Vec<T> are available with String as well, because
    String is actually implemented as a wrapper around a vector of bytes with some extra guarantees,
    restrictions, and capabilities. An example of a function that works the same way with Vec<T> and
    String is the new function to create an instance, shown in Listing 8-11.
     */

    let mut s = String::new();

    /*
    This line creates a new empty string called s, which we can then load data into. Often, we’ll
    have some initial data that we want to start the string with. For that, we use the to_string
    method, which is available on any type that implements the Display trait, as string literals do.
    Listing 8-12 shows two examples.
     */

    let data = "initial contents";

    let s = data.to_string();

    // the method also works on a literal directly:
    let s = "initial contents".to_string();

    /*
    Because strings are used for so many things, we can use many different generic APIs for strings,
    providing us with a lot of options. Some of them can seem redundant, but they all have their
    place! In this case, String::from and to_string do the same thing, so which you choose is a
    matter of style and readability.

    Remember that strings are UTF-8 encoded, so we can include any properly encoded data in them, as
    shown in Listing 8-14.
     */

    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    /*
    All of these are valid String values.
     */
}

/*
A String can grow in size and its contents can change, just like the contents of a Vec<T>, if you
push more data into it. In addition, you can conveniently use the + operator or the format! macro to
concatenate String values.
 */

fn appending_to_a_string_with_push_str_and_push() {
    /*
    We can grow a String by using the push_str method to append a string slice.
     */

    let mut s = String::from("foo");
    s.push_str("bar");

    /*
    After these two lines, s will contain foobar. The push_str method takes a string slice because
    we don’t necessarily want to take ownership of the parameter. For example, in the code in
    Listing 8-16, we want to be able to use s2 after appending its contents to s1.
     */

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s1 is {s1} | s2 is {s2}");

    /*
    f the push_str method took ownership of s2, we wouldn’t be able to print its value on the last
    line. However, this code works as we’d expect!

    The push method takes a single character as a parameter and adds it to the String. Listing 8-17
    adds the letter “l” to a String using the push method.
     */

    let mut s = String::from("lo");
    s.push('l');

    println!("s i {s}");
}

fn concatenation_with_plus_operator_or_the_format_macro() {
    /*
    Often, you’ll want to combine two existing strings. One way to do so is to use the + operator,
    as shown in Listing 8-18.
     */

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    println!("s2 = {s2} | s3 = {s3}");

    /*
    The string s3 will contain Hello, world!. The reason s1 is no longer valid after the addition,
    and the reason we used a reference to s2, has to do with the signature of the method that’s
    called when we use the + operator. The + operator uses the add method, whose signature looks
    something like this:

    fn add(self, s: &str) -> String {

    In the standard library, you'll see add defined using generics and associated types. Here, we’ve
    substituted in concrete types, which is what happens when we call this method with String
    values. We’ll discuss generics in Chapter 10. This signature gives us the clues we need to
    understand the tricky bits of the + operator.

    First, s2 has an &, meaning that we’re adding a reference of the second string to the first
    string. This is because of the s parameter in the add function: we can only add a &str to a
    String; we can’t add two String values together. But wait—the type of &s2 is &String, not &str,
    as specified in the second parameter to add. So why does Listing 8-18 compile?

    The reason we’re able to use &s2 in the call to add is that the compiler can coerce the &String
    argument into a &str. When we call the add method, Rust uses a deref coercion, which here turns
    &s2 into &s2[..]. We’ll discuss deref coercion in more depth in Chapter 15. Because add does not
    take ownership of the s parameter, s2 will still be a valid String after this operation.

    Second, we can see in the signature that add takes ownership of self, because self does not have
    an &. This means s1 in Listing 8-18 will be moved into the add call and will no longer be valid
    after that. So although let s3 = s1 + &s2; looks like it will copy both strings and create a new
    one, this statement actually takes ownership of s1, appends a copy of the contents of s2, and
    then returns ownership of the result. In other words, it looks like it’s making a lot of copies
    but isn’t; the implementation is more efficient than copying.

    If we need to concatenate multiple strings, the behavior of the + operator gets unwieldy:
     */

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("s = {s}");

    /*
    At this point, s will be tic-tac-toe. With all of the + and " characters, it’s difficult to see
    what’s going on. For more complicated string combining, we can instead use the format! macro:
     */

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
    println!("s = {s}");

    /*
    This code also sets s to tic-tac-toe. The format! macro works like println!, but instead of
    printing the output to the screen, it returns a String with the contents. The version of the
    code using format! is much easier to read, and the code generated by the format! macro uses
    references so that this call doesn’t take ownership of any of its parameters.
     */
}

fn indexing_into_strings() {
    /*
    In many other programming languages, accessing individual characters in a string by referencing
    them by index is a valid and common operation. However, if you try to access parts of a String
    using indexing syntax in Rust, you’ll get an error.

    Rust strings don’t support indexing. But why not? To answer that question, we need to discuss
    how Rust stores strings in memory.

    A String is a wrapper over a Vec<u8>. Let’s look at some of our properly encoded UTF-8 example
    strings from Listing 8-14. First, this one:
     */

    let hello = String::from("Hola");

    /*
    In this case, len will be 4, which means the vector storing the string “Hola” is 4 bytes long.
    Each of these letters takes 1 byte when encoded in UTF-8. The following line, however, may
    surprise you. (Note that this string begins with the capital Cyrillic letter Ze, not the number
    3.)
     */

    let hello = String::from("Здравствуйте");

    /*
    Asked how long the string is, you might say 12. In fact, Rust’s answer is 24: that’s the number
    of bytes it takes to encode “Здравствуйте” in UTF-8, because each Unicode scalar value in that
    string takes 2 bytes of storage. Therefore, an index into the string’s bytes will not always
    correlate to a valid Unicode scalar value. To demonstrate, consider this invalid Rust code:
     */

    let hello = "Здравствуйте";
    // let answer = &hello[0];

    /*
    You already know that answer will not be З, the first letter. When encoded in UTF-8, the first
    byte of З is 208 and the second is 151, so it would seem that answer should in fact be 208, but
    208 is not a valid character on its own. Returning 208 is likely not what a user would want if
    they asked for the first letter of this string; however, that’s the only data that Rust has at
    byte index 0. Users generally don’t want the byte value returned, even if the string contains
    only Latin letters: if &"hello"[0] were valid code that returned the byte value, it would return
    104, not h.

    The answer, then, is that to avoid returning an unexpected value and causing bugs that might not
    be discovered immediately, Rust doesn’t compile this code at all and prevents misunderstandings
    early in the development process.

    Another point about UTF-8 is that there are actually three relevant ways to look at strings from
    Rust’s perspective: as bytes, scalar values, and grapheme clusters (the closest thing to what we
    would call letters).

    If we look at the Hindi word “नमस्ते” written in the Devanagari script, it is stored as a vector of
    u8 values that looks like this:

    [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]

    That’s 18 bytes and is how computers ultimately store this data. If we look at them as Unicode
    scalar values, which are what Rust’s char type is, those bytes look like this:

    ['न', 'म', 'स', '्', 'त', 'े']

    There are six char values here, but the fourth and sixth are not letters: they’re diacritics
    that don’t make sense on their own. Finally, if we look at them as grapheme clusters, we’d get
    what a person would call the four letters that make up the Hindi word:

    ["न", "म", "स्", "ते"]

    Rust provides different ways of interpreting the raw string data that computers store so that
    each program can choose the interpretation it needs, no matter what human language the data is
    in.

    A final reason Rust doesn’t allow us to index into a String to get a character is that indexing
    operations are expected to always take constant time (O(1)). But it isn’t possible to guarantee
    that performance with a String, because Rust would have to walk through the contents from the
    beginning to the index to determine how many valid characters there were.
     */
}

fn slicing_strings() {
    /*
    Indexing into a string is often a bad idea because it’s not clear what the return type of the
    string-indexing operation should be: a byte value, a character, a grapheme cluster, or a string
    slice. If you really need to use indices to create string slices, therefore, Rust asks you to be
    more specific.

    Rather than indexing using [] with a single number, you can use [] with a range to create a
    string slice containing particular bytes:
     */

    let hello = "Здравствуйте";

    let s = &hello[0..4];
    println!("s = {s}");

    /*
    Here, s will be a &str that contains the first 4 bytes of the string. Earlier, we mentioned that
    each of these characters was 2 bytes, which means s will be Зд.

    If we were to try to slice only part of a character’s bytes with something like &hello[0..1],
    Rust would panic at runtime in the same way as if an invalid index were accessed in a vector:

    $ cargo run
        Compiling collections v0.1.0 (file:///projects/collections)
         Finished dev [unoptimized + debuginfo] target(s) in 0.43s
          Running `target/debug/collections`
    thread 'main' panicked at 'byte index 1 is not a char boundary; it is inside 'З' (bytes 0..2) of
     `Здравствуйте`', src/main.rs:4:14
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

    You should use ranges to create string slices with caution, because doing so can crash your
    program.
     */
}

fn methods_for_iterating_over_strings() {
    /*
    The best way to operate on pieces of strings is to be explicit about whether you want characters
    or bytes. For individual Unicode scalar values, use the chars method. Calling chars on “Зд”
    separates out and returns two values of type char, and you can iterate over the result to access
    each element:
     */

    for c in "Зд".chars() {
        println!("{c}");
    }

    /*
    This code will print the following:

    З
    д
     */

    /*
    Alternatively, the bytes method returns each raw byte, which might be appropriate for your
    domain:
     */

    for b in "Зд".bytes() {
        println!("{b}");
    }

    /*
    208
    151
    208
    180
     */

    /*
    But be sure to remember that valid Unicode scalar values may be made up of more than 1 byte.

    Getting grapheme clusters from strings as with the Devanagari script is complex, so this
    functionality is not provided by the standard library. Crates are available on crates.io if this
    is the functionality you need.
     */
}

/*
To summarize, strings are complicated. Different programming languages make different choices about
how to present this complexity to the programmer. Rust has chosen to make the correct handling of
String data the default behavior for all Rust programs, which means programmers have to put more
thought into handling UTF-8 data upfront. This trade-off exposes more of the complexity of strings
than is apparent in other programming languages, but it prevents you from having to handle errors
involving non-ASCII characters later in your development life cycle.

The good news is that the standard library offers a lot of functionality built off the String and
&str types to help handle these complex situations correctly. Be sure to check out the documentation
for useful methods like contains for searching in a string and replace for substituting parts of a
string with another string.

Let’s switch to something a bit less complex: hash maps!
 */

fn main() {
    slicing_strings();
}
