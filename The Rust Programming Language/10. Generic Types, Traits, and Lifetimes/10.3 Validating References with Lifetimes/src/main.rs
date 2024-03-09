/*
Lifetimes are another kind of generic that we’ve already been using. Rather than ensuring that a
type has the behavior we want, lifetimes ensure that references are valid as long as we need them to
be.

One detail we didn’t discuss in the “References and Borrowing” section in Chapter 4 is that every
reference in Rust has a lifetime, which is the scope for which that reference is valid. Most of the
time, lifetimes are implicit and inferred, just like most of the time, types are inferred. We must
only annotate types when multiple types are possible. In a similar way, we must annotate lifetimes
when the lifetimes of references could be related in a few different ways. Rust requires us to
annotate the relationships using generic lifetime parameters to ensure the actual references used at
runtime will definitely be valid.

Annotating lifetimes is not a concept most other programming languages have, so this is going to
feel unfamiliar. Although we won’t cover lifetimes in their entirety in this chapter, we’ll discuss
common ways you might encounter lifetime syntax so you can get comfortable with the concept.
 */

fn preventing_dangling_references_with_lifetimes() {
    /*
    The main aim of lifetimes is to prevent dangling references, which cause a program to reference
    data other than the data it’s intended to reference. Consider the program in Listing 10-16,
    which has an outer scope and an inner scope.

    let r;

    {
        let x = 5;
        r = &x;
    }

    println!("r: {}", r);



    Note: The examples in Listings 10-16, 10-17, and 10-23 declare variables without giving them an
    initial value, so the variable name exists in the outer scope. At first glance, this might
    appear to be in conflict with Rust’s having no null values. However, if we try to use a variable
    before giving it a value, we’ll get a compile-time error, which shows that Rust indeed does not
    allow null values.

    The outer scope declares a variable named r with no initial value, and the inner scope declares
    a variable named x with the initial value of 5. Inside the inner scope, we attempt to set the
    value of r as a reference to x. Then the inner scope ends, and we attempt to print the value in
    r. This code won’t compile because what the value r is referring to has gone out of scope before
    we try to use it. Here is the error message:

    $ cargo run
       Compiling chapter10 v0.1.0 (file:///projects/chapter10)
    error[E0597]: `x` does not live long enough
     --> src/main.rs:6:13
      |
    6 |         r = &x;
      |             ^^ borrowed value does not live long enough
    7 |     }
      |     - `x` dropped here while still borrowed
    8 |
    9 |     println!("r: {}", r);
      |                       - borrow later used here

    For more information about this error, try `rustc --explain E0597`.
    error: could not compile `chapter10` due to previous error

    The variable x doesn’t “live long enough.” The reason is that x will be out of scope when the
    inner scope ends on line 7. But r is still valid for the outer scope; because its scope is
    larger, we say that it “lives longer.” If Rust allowed this code to work, r would be referencing
    memory that was deallocated when x went out of scope, and anything we tried to do with r
    wouldn’t work correctly. So how does Rust determine that this code is invalid? It uses a borrow
    checker.
     */
}

fn the_borrow_checker() {
    /*
    The Rust compiler has a borrow checker that compares scopes to determine whether all borrows are
    valid. Listing 10-17 shows the same code as Listing 10-16 but with annotations showing the
    lifetimes of the variables.
     */
    let r;                // ---------+-- 'a
    //          |
    {                           //          |
        let x = 5;         // -+-- 'b  |
        r = &x;                 //  |       |
    }                           // -+       |
    //          |
    println!("r: {}", r);       //          |
    // ---------+

    /*
    Here, we’ve annotated the lifetime of r with 'a and the lifetime of x with 'b. As you can see,
    the inner 'b block is much smaller than the outer 'a lifetime block. At compile time, Rust
    compares the size of the two lifetimes and sees that r has a lifetime of 'a but that it refers
    to memory with a lifetime of 'b. The program is rejected because 'b is shorter than 'a: the
    subject of the reference doesn’t live as long as the reference.

    Listing 10-18 fixes the code so it doesn’t have a dangling reference and compiles without any
    errors.
     */

    let x = 5;              // ----------+-- 'b
    //           |
    let r = &x;           // --+-- 'a  |
    //   |       |
    println!("r: {}", r);       //   |       |
    // --+       |
    // ----------+

    /*
    Here, x has the lifetime 'b, which in this case is larger than 'a. This means r can reference x
    because Rust knows that the reference in r will always be valid while x is valid.

    Now that you know where the lifetimes of references are and how Rust analyzes lifetimes to
    ensure references will always be valid, let’s explore generic lifetimes of parameters and return
    values in the context of functions.
     */
}

fn generic_lifetimes_in_functions() {
    /*
    We’ll write a function that returns the longer of two string slices. This function will take two
    string slices and return a single string slice. After we’ve implemented the longest function,
    the code in Listing 10-19 should print The longest string is abcd.


    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);


    Note that we want the function to take string slices, which are references, rather than strings,
    because we don’t want the longest function to take ownership of its parameters. Refer to the
    “String Slices as Parameters” section in Chapter 4 for more discussion about why the parameters
    we use in Listing 10-19 are the ones we want.

    If we try to implement the longest function as shown in Listing 10-20, it won’t compile.

    fn longest(x: &str, y: &str) -> &str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    Instead, we get the following error that talks about lifetimes:

    $ cargo run
       Compiling chapter10 v0.1.0 (file:///projects/chapter10)
    error[E0106]: missing lifetime specifier
     --> src/main.rs:9:33
      |
    9 | fn longest(x: &str, y: &str) -> &str {
      |               ----     ----     ^ expected named lifetime parameter
      |
      = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `x` or `y`
    help: consider introducing a named lifetime parameter
      |
    9 | fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
      |           ++++     ++          ++          ++

    For more information about this error, try `rustc --explain E0106`.
    error: could not compile `chapter10` due to previous error

    The help text reveals that the return type needs a generic lifetime parameter on it because Rust
    can’t tell whether the reference being returned refers to x or y. Actually, we don’t know
    either, because the if block in the body of this function returns a reference to x and the else
    block returns a reference to y!

    When we’re defining this function, we don’t know the concrete values that will be passed into
    this function, so we don’t know whether the if case or the else case will execute. We also don’t
    know the concrete lifetimes of the references that will be passed in, so we can’t look at the
    scopes as we did in Listings 10-17 and 10-18 to determine whether the reference we return will
    always be valid. The borrow checker can’t determine this either, because it doesn’t know how the
    lifetimes of x and y relate to the lifetime of the return value. To fix this error, we’ll add
    generic lifetime parameters that define the relationship between the references so the borrow
    checker can perform its analysis.
     */
}

fn lifetime_annotation_syntax() {
    /*
    Lifetime annotations don’t change how long any of the references live. Rather, they describe the
    relationships of the lifetimes of multiple references to each other without affecting the
    lifetimes. Just as functions can accept any type when the signature specifies a generic type
    parameter, functions can accept references with any lifetime by specifying a generic lifetime
    parameter.

    Lifetime annotations have a slightly unusual syntax: the names of lifetime parameters must start
    with an apostrophe (') and are usually all lowercase and very short, like generic types. Most
    people use the name 'a for the first lifetime annotation. We place lifetime parameter
    annotations after the & of a reference, using a space to separate the annotation from the
    reference’s type.

    Here are some examples: a reference to an i32 without a lifetime parameter, a reference to an
    i32 that has a lifetime parameter named 'a, and a mutable reference to an i32 that also has the
    lifetime 'a.

    &i32        // a reference
    &'a i32     // a reference with an explicit lifetime
    &'a mut i32 // a mutable reference with an explicit lifetime

    One lifetime annotation by itself doesn’t have much meaning, because the annotations are meant
    to tell Rust how generic lifetime parameters of multiple references relate to each other. Let’s
    examine how the lifetime annotations relate to each other in the context of the longest
    function.
     */
}

fn lifetime_annotations_in_function_signatures() {
    /*
    To use lifetime annotations in function signatures, we need to declare the generic lifetime
    parameters inside angle brackets between the function name and the parameter list, just as we
    did with generic type parameters.

    We want the signature to express the following constraint: the returned reference will be valid
    as long as both the parameters are valid. This is the relationship between lifetimes of the
    parameters and the return value. We’ll name the lifetime 'a and then add it to each reference,
    as shown in Listing 10-21.
     */

    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
    /*
    This code should compile and produce the result we want when we use it with the main function
    in Listing 10-19.

   The function signature now tells Rust that for some lifetime 'a, the function takes two
   parameters, both of which are string slices that live at least as long as lifetime 'a. The
   function signature also tells Rust that the string slice returned from the function will live at
   least as long as lifetime 'a. In practice, it means that the lifetime of the reference returned
   by the longest function is the same as the smaller of the lifetimes of the values referred to by
   the function arguments. These relationships are what we want Rust to use when analyzing this
   code.

   Remember, when we specify the lifetime parameters in this function signature, we’re not changing
   the lifetimes of any values passed in or returned. Rather, we’re specifying that the borrow
   checker should reject any values that don’t adhere to these constraints. Note that the longest
   function doesn’t need to know exactly how long x and y will live, only that some scope can be
   substituted for 'a that will satisfy this signature.

   When annotating lifetimes in functions, the annotations go in the function signature, not in the
   function body. The lifetime annotations become part of the contract of the function, much like
   the types in the signature. Having function signatures contain the lifetime contract means the
   analysis the Rust compiler does can be simpler. If there’s a problem with the way a function is
   annotated or the way it is called, the compiler errors can point to the part of our code and the
   constraints more precisely. If, instead, the Rust compiler made more inferences about what we
   intended the relationships of the lifetimes to be, the compiler might only be able to point to a
   use of our code many steps away from the cause of the problem.

   When we pass concrete references to longest, the concrete lifetime that is substituted for 'a is
   the part of the scope of x that overlaps with the scope of y. In other words, the generic
   lifetime 'a will get the concrete lifetime that is equal to the smaller of the lifetimes of x
   and y. Because we’ve annotated the returned reference with the same lifetime parameter 'a, the
   returned reference will also be valid for the length of the smaller of the lifetimes of x and y.

   Let’s look at how the lifetime annotations restrict the longest function by passing in
   references that have different concrete lifetimes. Listing 10-22 is a straightforward example.
     */

    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

    /*
    In this example, string1 is valid until the end of the outer scope, string2 is valid until the
    end of the inner scope, and result references something that is valid until the end of the inner
    scope. Run this code, and you’ll see that the borrow checker approves; it will compile and print
    The longest string is long string is long.

    Next, let’s try an example that shows that the lifetime of the reference in result must be the
    smaller lifetime of the two arguments. We’ll move the declaration of the result variable outside
    the inner scope but leave the assignment of the value to the result variable inside the scope
    with string2. Then we’ll move the println! that uses result to outside the inner scope, after
    the inner scope has ended. The code in Listing 10-23 will not compile.

    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);

    When we try to compile this code, we get this error:

    $ cargo run
       Compiling chapter10 v0.1.0 (file:///projects/chapter10)
    error[E0597]: `string2` does not live long enough
     --> src/main.rs:6:44
      |
    6 |         result = longest(string1.as_str(), string2.as_str());
      |                                            ^^^^^^^^^^^^^^^^ borrowed value does not live long enough
    7 |     }
      |     - `string2` dropped here while still borrowed
    8 |     println!("The longest string is {}", result);
      |                                          ------ borrow later used here

    For more information about this error, try `rustc --explain E0597`.
    error: could not compile `chapter10` due to previous error

    The error shows that for result to be valid for the println! statement, string2 would need to be
    valid until the end of the outer scope. Rust knows this because we annotated the lifetimes of
    the function parameters and return values using the same lifetime parameter 'a.

    As humans, we can look at this code and see that string1 is longer than string2 and therefore
    result will contain a reference to string1. Because string1 has not gone out of scope yet, a
    reference to string1 will still be valid for the println! statement. However, the compiler can’t
    see that the reference is valid in this case. We’ve told Rust that the lifetime of the reference
    returned by the longest function is the same as the smaller of the lifetimes of the references
    passed in. Therefore, the borrow checker disallows the code in Listing 10-23 as possibly having
    an invalid reference.

    Try designing more experiments that vary the values and lifetimes of the references passed in to
    the longest function and how the returned reference is used. Make hypotheses about whether or
    not your experiments will pass the borrow checker before you compile; then check to see if
    you’re right!
     */
}

fn thinking_in_terms_of_lifetimes() {
    /*
    The way in which you need to specify lifetime parameters depends on what your function is doing.
    For example, if we changed the implementation of the longest function to always return the first
    parameter rather than the longest string slice, we wouldn’t need to specify a lifetime on the y
    parameter. The following code will compile:
     */

    fn longest<'a>(x: &'a str, y: &str) -> &'a str {
        x
    }

    /*
    We’ve specified a lifetime parameter 'a for the parameter x and the return type, but not for the
    parameter y, because the lifetime of y does not have any relationship with the lifetime of x or
    the return value.

    When returning a reference from a function, the lifetime parameter for the return type needs to
    match the lifetime parameter for one of the parameters. If the reference returned does not refer
    to one of the parameters, it must refer to a value created within this function. However, this
    would be a dangling reference because the value will go out of scope at the end of the function.
    Consider this attempted implementation of the longest function that won’t compile:

    fn longest<'a>(x: &str, y: &str) -> &'a str {
        let result = String::from("really long string");
        result.as_str()
    }

    Here, even though we’ve specified a lifetime parameter 'a for the return type, this
    implementation will fail to compile because the return value lifetime is not related to the
    lifetime of the parameters at all. Here is the error message we get:


    $ cargo run
       Compiling chapter10 v0.1.0 (file:///projects/chapter10)
    error[E0515]: cannot return reference to local variable `result`
      --> src/main.rs:11:5
       |
    11 |     result.as_str()
       |     ^^^^^^^^^^^^^^^ returns a reference to data owned by the current function

    For more information about this error, try `rustc --explain E0515`.
    error: could not compile `chapter10` due to previous error


    The problem is that result goes out of scope and gets cleaned up at the end of the longest
    function. We’re also trying to return a reference to result from the function. There is no way
    we can specify lifetime parameters that would change the dangling reference, and Rust won’t let
    us create a dangling reference. In this case, the best fix would be to return an owned data type
    rather than a reference so the calling function is then responsible for cleaning up the value.

    Ultimately, lifetime syntax is about connecting the lifetimes of various parameters and return
    values of functions. Once they’re connected, Rust has enough information to allow memory-safe
    operations and disallow operations that would create dangling pointers or otherwise violate
    memory safety.
     */
}

fn lifetime_annotations_in_struct_definitions() {
    /*
    So far, the structs we’ve defined all hold owned types. We can define structs to hold
    references, but in that case we would need to add a lifetime annotation on every reference in
    the struct’s definition. Listing 10-24 has a struct named ImportantExcerpt that holds a string
    slice.
     */

    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    /*
    This struct has the single field part that holds a string slice, which is a reference. As with
    generic data types, we declare the name of the generic lifetime parameter inside angle brackets
    after the name of the struct so we can use the lifetime parameter in the body of the struct
    definition. This annotation means an instance of ImportantExcerpt can’t outlive the reference it
    holds in its part field.

    The main function here creates an instance of the ImportantExcerpt struct that holds a reference
    to the first sentence of the String owned by the variable novel. The data in novel exists before
    the ImportantExcerpt instance is created. In addition, novel doesn’t go out of scope until after
    the ImportantExcerpt goes out of scope, so the reference in the ImportantExcerpt instance is
    valid.
     */
}

fn lifetime_elision() {
    /*
    You’ve learned that every reference has a lifetime and that you need to specify lifetime
    parameters for functions or structs that use references. However, in Chapter 4 we had a function
    in Listing 4-9, shown again in Listing 10-25, that compiled without lifetime annotations.
     */

    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }

        &s[..]
    }

    /*
    The reason this function compiles without lifetime annotations is historical: in early versions
    (pre-1.0) of Rust, this code wouldn’t have compiled because every reference needed an explicit
    lifetime. At that time, the function signature would have been written like this:

    fn first_word<'a>(s: &'a str) -> &'a str {

    After writing a lot of Rust code, the Rust team found that Rust programmers were entering the
    same lifetime annotations over and over in particular situations. These situations were
    predictable and followed a few deterministic patterns. The developers programmed these patterns
    into the compiler’s code so the borrow checker could infer the lifetimes in these situations and
    wouldn’t need explicit annotations.

    This piece of Rust history is relevant because it’s possible that more deterministic patterns
    will emerge and be added to the compiler. In the future, even fewer lifetime annotations might
    be required.

    The patterns programmed into Rust’s analysis of references are called the lifetime elision
    rules. These aren’t rules for programmers to follow; they’re a set of particular cases that the
    compiler will consider, and if your code fits these cases, you don’t need to write the lifetimes
    explicitly.

    The elision rules don’t provide full inference. If Rust deterministically applies the rules but
    there is still ambiguity as to what lifetimes the references have, the compiler won’t guess what
    the lifetime of the remaining references should be. Instead of guessing, the compiler will give
    you an error that you can resolve by adding the lifetime annotations.

    Lifetimes on function or method parameters are called input lifetimes, and lifetimes on return
    values are called output lifetimes.

    The compiler uses three rules to figure out the lifetimes of the references when there aren’t
    explicit annotations. The first rule applies to input lifetimes, and the second and third rules
    apply to output lifetimes. If the compiler gets to the end of the three rules and there are
    still references for which it can’t figure out lifetimes, the compiler will stop with an error.
    These rules apply to fn definitions as well as impl blocks.

    The first rule is that the compiler assigns a lifetime parameter to each parameter that’s a
    reference. In other words, a function with one parameter gets one lifetime parameter:
    fn foo<'a>(x: &'a i32); a function with two parameters gets two separate lifetime parameters:
    fn foo<'a, 'b>(x: &'a i32, y: &'b i32); and so on.

    The second rule is that, if there is exactly one input lifetime parameter, that lifetime is
    assigned to all output lifetime parameters: fn foo<'a>(x: &'a i32) -> &'a i32.

    The third rule is that, if there are multiple input lifetime parameters, but one of them is
    &self or &mut self because this is a method, the lifetime of self is assigned to all output
    lifetime parameters. This third rule makes methods much nicer to read and write because fewer
    symbols are necessary.

    Let’s pretend we’re the compiler. We’ll apply these rules to figure out the lifetimes of the
    references in the signature of the first_word function in Listing 10-25. The signature starts
    without any lifetimes associated with the references:

    fn first_word(s: &str) -> &str {

    Then the compiler applies the first rule, which specifies that each parameter gets its own
    lifetime. We’ll call it 'a as usual, so now the signature is this:

    fn first_word<'a>(s: &'a str) -> &str {

    The second rule applies because there is exactly one input lifetime. The second rule specifies
    that the lifetime of the one input parameter gets assigned to the output lifetime, so the
    signature is now this:

    fn first_word<'a>(s: &'a str) -> &'a str {

    Now all the references in this function signature have lifetimes, and the compiler can continue
    its analysis without needing the programmer to annotate the lifetimes in this function
    signature.

    Let’s look at another example, this time using the longest function that had no lifetime
    parameters when we started working with it in Listing 10-20:

    fn longest(x: &str, y: &str) -> &str {

    Let’s apply the first rule: each parameter gets its own lifetime. This time we have two
    parameters instead of one, so we have two lifetimes:

    fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str {

    You can see that the second rule doesn’t apply because there is more than one input lifetime.
    The third rule doesn’t apply either, because longest is a function rather than a method, so none
    of the parameters are self. After working through all three rules, we still haven’t figured out
    what the return type’s lifetime is. This is why we got an error trying to compile the code in
    Listing 10-20: the compiler worked through the lifetime elision rules but still couldn’t figure
    out all the lifetimes of the references in the signature.

    Because the third rule really only applies in method signatures, we’ll look at lifetimes in that
    context next to see why the third rule means we don’t have to annotate lifetimes in method
    signatures very often.
     */
}

fn lifetime_annotations_in_method_definition() {
    /*
    When we implement methods on a struct with lifetimes, we use the same syntax as that of generic
    type parameters shown in Listing 10-11. Where we declare and use the lifetime parameters depends
    on whether they’re related to the struct fields or the method parameters and return values.

    Lifetime names for struct fields always need to be declared after the impl keyword and then used
    after the struct’s name, because those lifetimes are part of the struct’s type.

    In method signatures inside the impl block, references might be tied to the lifetime of
    references in the struct’s fields, or they might be independent. In addition, the lifetime
    elision rules often make it so that lifetime annotations aren’t necessary in method signatures.
    Let’s look at some examples using the struct named ImportantExcerpt that we defined in Listing
    10-24.

    First, we’ll use a method named level whose only parameter is a reference to self and whose
    return value is an i32, which is not a reference to anything:

    impl<'a> ImportantExcerpt<'a> {
        fn level(&self) -> i32 {
            3
        }
    }

    The lifetime parameter declaration after impl and its use after the type name are required, but
    we’re not required to annotate the lifetime of the reference to self because of the first
    elision rule.

    Here is an example where the third lifetime elision rule applies:

    impl<'a> ImportantExcerpt<'a> {
        fn announce_and_return_part(&self, announcement: &str) -> &str {
            println!("Attention please: {}", announcement);
            self.part
        }
    }

    There are two input lifetimes, so Rust applies the first lifetime elision rule and gives both
    &self and announcement their own lifetimes. Then, because one of the parameters is &self, the
    return type gets the lifetime of &self, and all lifetimes have been accounted for.
     */
}

fn the_static_lifetime() {
    /*
    One special lifetime we need to discuss is 'static, which denotes that the affected reference
    can live for the entire duration of the program. All string literals have the 'static lifetime,
    which we can annotate as follows:
     */

    let s: &'static str = "I have a static lifetime.";

    /*
    The text of this string is stored directly in the program’s binary, which is always available.
    Therefore, the lifetime of all string literals is 'static.

    You might see suggestions to use the 'static lifetime in error messages. But before specifying
    'static as the lifetime for a reference, think about whether the reference you have actually
    lives the entire lifetime of your program or not, and whether you want it to. Most of the time,
    an error message suggesting the 'static lifetime results from attempting to create a dangling
    reference or a mismatch of the available lifetimes. In such cases, the solution is fixing those
    problems, not specifying the 'static lifetime.
     */
}

fn generic_type_parameters_trait_bounds_and_lifetimes_together() {
    /*
    Let’s briefly look at the syntax of specifying generic type parameters, trait bounds, and
    lifetimes all in one function!
     */

    use std::fmt::Display;

    fn longest_with_an_announcement<'a, T>(
        x: &'a str,
        y: &'a str,
        ann: T,
    ) -> &'a str
        where
            T: Display,
    {
        println!("Announcement! {}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    /*
    This is the longest function from Listing 10-21 that returns the longer of two string slices.
    But now it has an extra parameter named ann of the generic type T, which can be filled in by any
    type that implements the Display trait as specified by the where clause. This extra parameter
    will be printed using {}, which is why the Display trait bound is necessary. Because lifetimes
    are a type of generic, the declarations of the lifetime parameter 'a and the generic type
    parameter T go in the same list inside the angle brackets after the function name.
     */
}

/*
Summary

We covered a lot in this chapter! Now that you know about generic type parameters, traits and trait
bounds, and generic lifetime parameters, you’re ready to write code without repetition that works in
many different situations. Generic type parameters let you apply the code to different types. Traits
and trait bounds ensure that even though the types are generic, they’ll have the behavior the code
needs. You learned how to use lifetime annotations to ensure that this flexible code won’t have any
dangling references. And all of this analysis happens at compile time, which doesn’t affect runtime
performance!

Believe it or not, there is much more to learn on the topics we discussed in this chapter: Chapter
17 discusses trait objects, which are another way to use traits. There are also more complex
scenarios involving lifetime annotations that you will only need in very advanced scenarios; for
those, you should read the Rust Reference. But next, you’ll learn how to write tests in Rust so you
can make sure your code is working the way it should.
 */

fn main() {
    println!("Hello, world!");
}
