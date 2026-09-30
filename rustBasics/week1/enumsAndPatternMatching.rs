enum IPAddrType {
    V4(String),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn some_function() {
        println!("Lets get Rusty!");
    }
}

struct IpAddr {
    kind: IPAddrType,
    address: String,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quater,
}

fn main() {
    let localhost = IPAddrType::V4(String::from("127.0.0.1"));
    let some_number = Some(5);
    let some_string = Some("Hello");
    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y.unwrap_or(0);

    println!("{}", sum);

    let five = Some(5);
    let six = plus_one(five).unwrap_or(0);
    let none = plus_one(None).unwrap_or(0);

    println!("{} {} {}", five.unwrap_or(0), six, none);

    let some_value = Some(3);
    match some_value {
        Some(3) => println!("Its three!"),
        _ => (),
    }
    if let Some(3) = some_value {
        println!("Its three."); // All other patterns are ignored.
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quater => 25,
    }
}
