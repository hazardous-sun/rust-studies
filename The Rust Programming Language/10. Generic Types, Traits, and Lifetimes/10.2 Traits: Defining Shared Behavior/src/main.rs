/*
A trait defines functionality a particular type has and can share with other types. We can use
traits to define shared behavior in an abstract way. We can use trait bounds to specify that a
generic type can be any type that has certain behavior.

Note: Traits are similar to a feature often called interfaces in other languages, although with some
differences.
 */

fn defining_a_trait() {
    /*
    A type’s behavior consists of the methods we can call on that type. Different types share the
    same behavior if we can call the same methods on all of those types. Trait definitions are a way
    to group method signatures together to define a set of behaviors necessary to accomplish some
    purpose.

    For example, let’s say we have multiple structs that hold various kinds and amounts of text: a
    NewsArticle struct that holds a news story filed in a particular location and a Tweet that can
    have at most 280 characters along with metadata that indicates whether it was a new tweet, a
    retweet, or a reply to another tweet.

    We want to make a media aggregator library crate named aggregator that can display summaries of
    data that might be stored in a NewsArticle or Tweet instance. To do this, we need a summary from
    each type, and we’ll request that summary by calling a summarize method on an instance. Listing
    10-12 shows the definition of a public Summary trait that expresses this behavior.
     */

    pub trait Summary {
        fn summarize(&self) -> String;
    }

    /*
    Here, we declare a trait using the trait keyword and then the trait’s name, which is Summary in
    this case. We’ve also declared the trait as pub so that crates depending on this crate can make
    use of this trait too, as we’ll see in a few examples. Inside the curly brackets, we declare the
    method signatures that describe the behaviors of the types that implement this trait, which in
    this case is fn summarize(&self) -> String.

    After the method signature, instead of providing an implementation within curly brackets, we use
    a semicolon. Each type implementing this trait must provide its own custom behavior for the body
    of the method. The compiler will enforce that any type that has the Summary trait will have the
    method summarize defined with this signature exactly.

    A trait can have multiple methods in its body: the method signatures are listed one per line and
    each line ends in a semicolon.
     */
}

fn implementing_a_trait_on_a_type() {
    /*
    Now that we’ve defined the desired signatures of the Summary trait’s methods, we can implement
    it on the types in our media aggregator. Listing 10-13 shows an implementation of the Summary
    trait on the NewsArticle struct that uses the headline, the author, and the location to create
    the return value of summarize. For the Tweet struct, we define summarize as the username
    followed by the entire text of the tweet, assuming that tweet content is already limited to 280
    characters.
     */

    pub trait Summary {
        fn summarize(&self) -> String;
    }

    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
    }

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }

    /*
    Implementing a trait on a type is similar to implementing regular methods. The difference is
    that after impl, we put the trait name we want to implement, then use the for keyword, and then
    specify the name of the type we want to implement the trait for. Within the impl block, we put
    the method signatures that the trait definition has defined. Instead of adding a semicolon after
    each signature, we use curly brackets and fill in the method body with the specific behavior
    that we want the methods of the trait to have for the particular type.

    Now that the library has implemented the Summary trait on NewsArticle and Tweet, users of the
    crate can call the trait methods on instances of NewsArticle and Tweet in the same way we call
    regular methods. The only difference is that the user must bring the trait into scope as well as
    the types. Here’s an example of how a binary crate could use our aggregator library crate:
     */

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    /*
    This code prints 1 new tweet: horse_ebooks: of course, as you probably already know, people.

    Other crates that depend on the aggregator crate can also bring the Summary trait into scope to
    implement Summary on their own types. One restriction to note is that we can implement a trait
    on a type only if at least one of the trait or the type is local to our crate. For example, we
    can implement standard library traits like Display on a custom type like Tweet as part of our
    aggregator crate functionality, because the type Tweet is local to our aggregator crate. We can
    also implement Summary on Vec<T> in our aggregator crate, because the trait Summary is local to
    our aggregator crate.

    But we can’t implement external traits on external types. For example, we can’t implement the
    Display trait on Vec<T> within our aggregator crate, because Display and Vec<T> are both defined
    in the standard library and aren’t local to our aggregator crate. This restriction is part of a
    property called coherence, and more specifically the orphan rule, so named because the parent
    type is not present. This rule ensures that other people’s code can’t break your code and vice
    versa. Without the rule, two crates could implement the same trait for the same type, and Rust
    wouldn’t know which implementation to use.
     */
}

fn default_implementations() {
    /*
    Sometimes it’s useful to have default behavior for some or all of the methods in a trait instead
    of requiring implementations for all methods on every type. Then, as we implement the trait on a
    particular type, we can keep or override each method’s default behavior.

    In Listing 10-14 we specify a default string for the summarize method of the Summary trait
    instead of only defining the method signature, as we did in Listing 10-12.
     */

    pub trait Summary {
        fn summarize(&self) -> String {
            String::from("(Read more...)")
        }
    }

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());

    /*
    This code prints New article available! (Read more...).

    Creating a default implementation doesn’t require us to change anything about the implementation
    of Summary on Tweet in Listing 10-13. The reason is that the syntax for overriding a default
    implementation is the same as the syntax for implementing a trait method that doesn’t have a
    default implementation.

    Default implementations can call other methods in the same trait, even if those other methods
    don’t have a default implementation. In this way, a trait can provide a lot of useful
    functionality and only require implementors to specify a small part of it. For example, we could
    define the Summary trait to have a summarize_author method whose implementation is required, and
    then define a summarize method that has a default implementation that calls the summarize_author
    method:
     */
}

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

/*
To use this version of Summary, we only need to define summarize_author when we implement the trait
on a type:
 */

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

/*
After we define summarize_author, we can call summarize on instances of the Tweet struct, and the
default implementation of summarize will call the definition of summarize_author that we’ve
provided. Because we’ve implemented summarize_author, the Summary trait has given us the behavior of
the summarize method without requiring us to write any more code.
 */

fn foo() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    /*
    This code prints 1 new tweet: (Read more from @horse_ebooks...).

    Note that it isn’t possible to call the default implementation from an overriding implementation
    of that same method.
     */
}

fn traits_as_parameters() {
    /*
    Now that you know how to define and implement traits, we can explore how to use traits to define
    functions that accept many different types. We'll use the Summary trait we implemented on the
    NewsArticle and Tweet types in Listing 10-13 to define a notify function that calls the
    summarize method on its item parameter, which is of some type that implements the Summary trait.
    To do this, we use the impl Trait syntax, like this:
     */

    pub fn notify(item: &impl Summary) {
        println!("Breaking news! {}", item.summarize());
    }

    /*
    Instead of a concrete type for the item parameter, we specify the impl keyword and the trait
    name. This parameter accepts any type that implements the specified trait. In the body of
    notify, we can call any methods on item that come from the Summary trait, such as summarize. We
    can call notify and pass in any instance of NewsArticle or Tweet. Code that calls the function
    with any other type, such as a String or an i32, won’t compile because those types don’t
    implement Summary.
     */
}

fn trait_bound_syntax() {
    /*
    The impl Trait syntax works for straightforward cases but is actually syntax sugar for a longer
    form known as a trait bound; it looks like this:
     */

    pub fn notify<T: Summary>(item: &T) {
        println!("Breaking news! {}", item.summarize());
    }

    /*
    This longer form is equivalent to the example in the previous section but is more verbose. We
    place trait bounds with the declaration of the generic type parameter after a colon and inside
    angle brackets.

    The impl Trait syntax is convenient and makes for more concise code in simple cases, while the
    fuller trait bound syntax can express more complexity in other cases. For example, we can have
    two parameters that implement Summary. Doing so with the impl Trait syntax looks like this:
     */

    pub fn notify2(item1: &impl Summary, item2: &impl Summary) {}

    /*
    Using impl Trait is appropriate if we want this function to allow item1 and item2 to have
    different types (as long as both types implement Summary). If we want to force both parameters
    to have the same type, however, we must use a trait bound, like this:
     */

    pub fn notify3<T: Summary>(item1: &T, item2: &T) {}

    /*
    The generic type T specified as the type of the item1 and item2 parameters constrains the
    function such that the concrete type of the value passed as an argument for item1 and item2 must
    be the same.
     */
}

fn specify_multiple_trait_bounds() {
    /*
    We can also specify more than one trait bound. Say we wanted notify to use display formatting as
    well as summarize on item: we specify in the notify definition that item must implement both
    Display and Summary. We can do so using the + syntax:
     */

    pub fn notify(item: &(impl Summary + Display)) {}

    /*
    The + syntax is also valid with trait bounds on generic types:
     */

    pub fn notify2<T: Summary + Display>(item: &T) {}

    /*
    With the two trait bounds specified, the body of notify can call summarize and use {} to format
    item.
     */
}

fn clearer_trait_bounds_with_where_clauses() {
    /*
    Using too many trait bounds has its downsides. Each generic has its own trait bounds, so
    functions with multiple generic type parameters can contain lots of trait bound information
    between the function’s name and its parameter list, making the function signature hard to read.
    For this reason, Rust has alternate syntax for specifying trait bounds inside a where clause
    after the function signature. So instead of writing this:
     */

    fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}

    /*
    We can use a where clause, like this:
     */

    fn some_function2<T, U>(t: &T, u: &U) -> i32
        where
            T: Display + Clone,
            U: Clone + Debug,
    {}

    /*
    This function’s signature is less cluttered: the function name, parameter list, and return type
    are close together, similar to a function without lots of trait bounds.
     */
}

fn returning_types_that_implement_traits() {
    /*
    We can also use the impl Trait syntax in the return position to return a value of some type that
    implements a trait, as shown here:
     */

    fn returns_summarizable() -> impl Summary {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            retweet: false,
        }
    }

    /*
    By using impl Summary for the return type, we specify that the returns_summarizable function
    returns some type that implements the Summary trait without naming the concrete type. In this
    case, returns_summarizable returns a Tweet, but the code calling this function doesn’t need to
    know that.

    The ability to specify a return type only by the trait it implements is especially useful in the
    context of closures and iterators, which we cover in Chapter 13. Closures and iterators create
    types that only the compiler knows or types that are very long to specify. The impl Trait syntax
    lets you concisely specify that a function returns some type that implements the Iterator trait
    without needing to write out a very long type.

    However, you can only use impl Trait if you’re returning a single type. For example, this code
    that returns either a NewsArticle or a Tweet with the return type specified as impl Summary
    wouldn’t work:
     */

    fn returns_summarizable2(switch: bool) -> impl Summary {
        if switch {
            NewsArticle {
                headline: String::from(
                    "Penguins win the Stanley Cup Championship!",
                ),
                location: String::from("Pittsburgh, PA, USA"),
                author: String::from("Iceburgh"),
                content: String::from(
                    "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
                ),
            }
        } else {
            Tweet {
                username: String::from("horse_ebooks"),
                content: String::from(
                    "of course, as you probably already know, people",
                ),
                reply: false,
                retweet: false,
            }
        }
    }

    /*
    Returning either a NewsArticle or a Tweet isn’t allowed due to restrictions around how the impl
    Trait syntax is implemented in the compiler. We’ll cover how to write a function with this
    behavior in the “Using Trait Objects That Allow for Values of Different Types” section of
    Chapter 17.
     */
}

fn using_trait_bounds_to_conditionally_implement_methods() {
    /*
    By using a trait bound with an impl block that uses generic type parameters, we can implement
    methods conditionally for types that implement the specified traits. For example, the type
    Pair<T> in Listing 10-15 always implements the new function to return a new instance of Pair<T>
    (recall from the “Defining Methods” section of Chapter 5 that Self is a type alias for the type
    of the impl block, which in this case is Pair<T>). But in the next impl block, Pair<T> only
    implements the cmp_display method if its inner type T implements the PartialOrd trait that
    enables comparison and the Display trait that enables printing.
     */

    use std::fmt::Display;

    struct Pair<T> {
        x: T,
        y: T,
    }

    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
    }

    impl<T: Display + PartialOrd> Pair<T> {
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("The largest member is x = {}", self.x);
            } else {
                println!("The largest member is y = {}", self.y);
            }
        }
    }

    /*
    We can also conditionally implement a trait for any type that implements another trait.
    Implementations of a trait on any type that satisfies the trait bounds are called blanket
    implementations and are extensively used in the Rust standard library. For example, the standard
    library implements the ToString trait on any type that implements the Display trait. The impl
    block in the standard library looks similar to this code:
     */

    impl<T: Display> ToString for T {
        // --snip--
    }

    /*
    Because the standard library has this blanket implementation, we can call the to_string method
    defined by the ToString trait on any type that implements the Display trait. For example, we can
    turn integers into their corresponding String values like this because integers implement
    Display:
     */

    let s = 3.to_string();

    /*
    Blanket implementations appear in the documentation for the trait in the “Implementors” section.

    Traits and trait bounds let us write code that uses generic type parameters to reduce
    duplication but also specify to the compiler that we want the generic type to have particular
    behavior. The compiler can then use the trait bound information to check that all the concrete
    types used with our code provide the correct behavior. In dynamically typed languages, we would
    get an error at runtime if we called a method on a type which didn’t define the method. But Rust
    moves these errors to compile time so we’re forced to fix the problems before our code is even
    able to run. Additionally, we don’t have to write code that checks for behavior at runtime
    because we’ve already checked at compile time. Doing so improves performance without having to
    give up the flexibility of generics.
     */
}

fn main() {
    println!("Hello, world!");
}
