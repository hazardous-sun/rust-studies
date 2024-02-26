/*
The last of our common collections is the hash map. The type HashMap<K, V> stores a mapping of keys
of type K to values of type V using a hashing function, which determines how it places these keys
and values into memory. Many programming languages support this kind of data structure, but they
often use a different name, such as hash, map, object, hash table, dictionary, or associative array,
just to name a few.

Hash maps are useful when you want to look up data not by using an index, as you can with vectors,
but by using a key that can be of any type. For example, in a game, you could keep track of each
team’s score in a hash map in which each key is a team’s name and the values are each team’s score.
Given a team name, you can retrieve its score.

We’ll go over the basic API of hash maps in this section, but many more goodies are hiding in the
functions defined on HashMap<K, V> by the standard library. As always, check the standard library
documentation for more information.
 */

fn creating_a_new_hashmap() {
    /*
    One way to create an empty hash map is using new and adding elements with insert. In
    Listing 8-20, we’re keeping track of the scores of two teams whose names are Blue and Yellow.
    The Blue team starts with 10 points, and the Yellow team starts with 50.
     */

    use std::collections::HashMap; // Usually set to global scope

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
}

fn accessing_values_in_a_hash_map() {
    /*
    We can get a value out of the hash map by providing its key to the get method, as shown in
    Listing 8-21.
     */

    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    /*
    Here, score will have the value that’s associated with the Blue team, and the result will be 10.
    The get method returns an Option<&V>; if there’s no value for that key in the hash map, get will
    return None. This program handles the Option by calling copied to get an Option<i32> rather than
    an Option<&i32>, then unwrap_or to set score to zero if scores doesn't have an entry for the
    key.

    We can iterate over each key/value pair in a hash map in a similar manner as we do with vectors,
    using a for loop:
     */

    let mut scores = HashMap::new();

    let value = String::from("Green");

    scores.insert(String::from("Blue"), 10); // insert method takes ownership of the String
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

}

fn hash_maps_and_ownership() {
    /*
    For types that implement the Copy trait, like i32, the values are copied into the hash map. For
    owned values like String, the values will be moved and the hash map will be the owner of those
    values, as demonstrated in Listing 8-22.
     */

    use std::collections::HashMap;

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point

    /*
    We aren’t able to use the variables field_name and field_value after they’ve been moved into the
    hash map with the call to insert.

    If we insert references to values into the hash map, the values won’t be moved into the hash
    map. The values that the references point to must be valid for at least as long as the hash map
    is valid. We’ll talk more about these issues in the “Validating References with Lifetimes”
    section in Chapter 10.
     */
}

fn updating_a_hash_map() {
    /*
    Although the number of key and value pairs is growable, each unique key can only have one value
    associated with it at a time (but not vice versa: for example, both the Blue team and the Yellow
    team could have value 10 stored in the scores hash map).

    When you want to change the data in a hash map, you have to decide how to handle the case when a
    key already has a value assigned. You could replace the old value with the new value, completely
    disregarding the old value. You could keep the old value and ignore the new value, only adding
    the new value if the key doesn’t already have a value. Or you could combine the old value and
    the new value. Let’s look at how to do each of these!

    If we insert a key and a value into a hash map and then insert that same key with a different
    value, the value associated with that key will be replaced. Even though the code in Listing 8-23
    calls insert twice, the hash map will only contain one key/value pair because we’re inserting
    the value for the Blue team’s key both times.W
     */

    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);

    /*
    It’s common to check whether a particular key already exists in the hash map with a value then
    take the following actions: if the key does exist in the hash map, the existing value should
    remain the way it is. If the key doesn’t exist, insert it and a value for it.

    Hash maps have a special API for this called entry that takes the key you want to check as a
    parameter. The return value of the entry method is an enum called Entry that represents a value
    that might or might not exist. Let’s say we want to check whether the key for the Yellow team
    has a value associated with it. If it doesn’t, we want to insert the value 50, and the same for
    the Blue team. Using the entry API, the code looks like Listing 8-24.
     */

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    /*
    Another common use case for hash maps is to look up a key's value and then update it based on
    the old value. For instance, Listing 8-25 shows code that counts how many times each word
    appears in some text. We use a hash map with the words as keys and increment the value to keep
    track of how many times we’ve seen that word. If it’s the first time we’ve seen a word, we’ll
    first insert the value 0.
     */

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
}

fn main() {
    updating_a_hash_map();
}
