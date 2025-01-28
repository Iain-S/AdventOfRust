mod closures;

fn main() {
    ownership();
    let a = || 5;
    println!("{}", closures::takes_a_closure(a));
    return;
}

fn takes_a_closure() {
    let example_closure = |x: i32| x + 1;
    println!("{}", example_closure(5));

    // println!("Hello, world!");
    // println!("Total distance: {}", total_distance(vec![(0,0),(1,1)]));
}

fn ownership() {
    // 1. Each value has a variable that is its owner.
    // 2. There can only be one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped.
    {
        let s = "hello";
        println!("{}", s);
    }
    // println!("{}", s); // This will not work because s is out of scope.

    let five = 5;
    let ref_five = &five;
    println!("{}", ref_five);

    let mystr: &str = "hi";

    let x = 1;
    let y = x; // copy

    // Note that Rust does not copy heap data by default.
    let s1 = String::from("hellooo");
    let s2 = s1; // move (would be shallow copy in C++)
    let s3 = s2.clone(); // deep copy

    takes_ownership(s2);
    makes_copy(x);

    // println!("{}", s1); // This will not work because s1 was moved.
    println!("{}", x); // This will work because x was copied.

    println!("{}: {}", calculate_length(&s3), s3);

    // Mutable references
    // Only allowed to have one mutable reference to a particular piece of data in a particular scope.
    let mut s4 = String::from("yellow");
    change_length(&mut s4);
    println!("{}", s4);

    // Slices
    let mut ht = String::from("hi there");
    let hi = &ht[..2];
    let there = &ht[2..];
    // ht.clear();  // Not allowed because there are still references to the data.
    println!("{},{}", hi, there);
    ht.clear(); // Allowed because there are no references to the data left.
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

// Passing in a reference is known as borrowing.
// Note that Rust references are similar to C++ pointers.
fn calculate_length(s: &String) -> usize {
    let length = s.len();
    length
}

fn change_length(s: &mut String) {
    s.push_str(" submarine");
}

// fn first_word(s: &String) ->
