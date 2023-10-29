/*
Where structs give you a way of grouping together related fields and data, like a Rectangle with its
width and height, enums give you a way of saying a value is one of a possible set of values. For
example, we may want to say that Rectangle is one of a set of possible shapes that also includes
Circle and Triangle. To do this, Rust allows us to encode these possibilities as an enum.

Let’s look at a situation we might want to express in code and see why enums are useful and more
appropriate than structs in this case. Say we need to work with IP addresses. Currently, two major
standards are used for IP addresses: version four and version six. Because these are the only
possibilities for an IP address that our program will come across, we can enumerate all possible
variants, which is where enumeration gets its name.

Any IP address can be either a version four or a version six address, but not both at the same time.
That property of IP addresses makes the enum data structure appropriate because an enum value can
only be one of its variants. Both version four and version six addresses are still fundamentally IP
addresses, so they should be treated as the same type when the code is handling situations that
apply to any kind of IP address.

We can express this concept in code by defining an IpAddrKind enumeration and listing the possible
kinds an IP address can be, V4 and V6. These are the variants of the enum:
 */
/*
enum IpAddrKind {
    V4,
    V6
}
*/
/*
IpAddrKind is now a custom data type that we can use elsewhere in our code.

Enum values:

We can create instances of each of the two variants of  IpAddrKind like this:

let four = IpAddrKind::V4;
let six = IpAddrKind::V6;

Note that the variants of the enum are namespaced under its identifier, and we use a double colon to
separate the two. This is useful because now both values IpAddrKind::V4 and IpAddrKind::V6 are of
the same type: IpAddrKind. We can then, for instance, define a function that takes any IpAddrKind:
 */
/*
fn route(ip_kind: IpAddrKind) {}
*/
/*
And we can call this function with either variant:

route(IpAddrKind::V4);
route(IpAddrKind::V6);

Using enums has even more advantages. Thinking more about our IP address type, at the moment we
don't have a way to store the actual IP address data; we only know what kind it is. Given that you
just learned about structs in Chapter 5, you might be tempted to tackle this problem with structs as
shown in the following code:
 */

/*
fn main() {
    enum IpAddrKind {
        V4,
        V6
    }

    struct IpAddr {
        kind: IpAddrKind,
        address: String
    }

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1")
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1")
    };
}
*/

/*
Here, we’ve defined a struct IpAddr that has two fields: a kind field that is of type IpAddrKind
(the enum we defined previously) and an address field of type String. We have two instances of this
struct. The first is home, and it has the value IpAddrKind::V4 as its kind with associated address
data of 127.0.0.1. The second instance is loopback. It has the other variant of IpAddrKind as its
kind value, V6, and has address ::1 associated with it. We’ve used a struct to bundle the kind and
address values together, so now the variant is associated with the value.

However, representing the same concept using just an enum is more concise: rather than an enum
inside a struct, we can put data directly into each enum variant. This new definition of the IpAddr
enum says that both V4 and V6 variants will have associated String values:
 */

fn main() {
    enum IpAddr {
        V4(String),
        V6(String),
    }

    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1"));
}