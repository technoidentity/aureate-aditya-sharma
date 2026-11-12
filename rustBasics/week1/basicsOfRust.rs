use std::io;

fn main() {
    let x = 5; // Without mut we cannot reassign the variable x.
    println!("The value of x is {}", x);
    let x = "Six"; // We can shadow the last declared x value by declaring it again without using the mut keyword.
    println!("The changed value for x is {}", x);

    // The concept of constant values
    // const SUB_COUNT: u32 = 100_000; // We cannot mutate a const.
    // They always have a type.

    // Scalar datatypes
    // Integers
    // Floating point numbers
    // Booleans
    // Character
    // Compound Datatypes
    // Fixed size arrays that contains related data that might be of different type are tuples.
    // We can pick the values out of the tuples by destructuring and by dot notation.
    let tup = ("My Name is astrophyfreak.", 24);
    let (intro, lucky_number) = tup;
    println!(
        "This is my intro: {} And my lucky number is: {} ",
        intro, lucky_number
    );
    let intro_again = tup.0;
    let again_lucky_num = tup.1;
    println!(
        "I am not going to write the whole: {} :: {}",
        intro_again, again_lucky_num
    );

    // We can also declare the arrays instead of tuples as a comma separated list enclosed in [].
    // If we want something that can change size dynamically then we use vectors.
    let err_codes = [200, 404, 500];
    let _not_found = err_codes[1];
    let _byte = [0; 8]; // array of size 8 having all 0s.
    new_function();
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read the input.");
    // read_line includes the Enter key's newline, so the input we get is "YES\n" like this.
    let choice = choice.trim();
    if choice == "YES" {
        println!("Wow! Thanks mate.")
    } else if choice == "NO" {
        println!("Oh! Leave it.");
    } else {
        println!("Type YES or NO case sensitive next time. Bye!");
    }
    // In rust these conditions must be a boolean.

    let mut x = 4;
    let y = 6;
    println!("The value of the new pair sum is: {}", add(x, y));

    let z = loop {
        println!("Lets get rusty!");
        if x == 10 {
            break x; // we can actually make the loop an expression. It reuturns something.
        }
        x += 1;
    };
    println!("The value of z is: {}", z);
    /* 
     * Multiline comment
     *  we have a classic while loop too
     * we also have a for in loop
    */
    for number in 1..4 {
        println!("{}", number);
    }
}

fn new_function() {
    println!("This is my new function! Do you like it ?");
}

fn add(x: i32, y: i32) -> i32 {
    return x + y;
}
