use std::io;

fn main() {
    println!("Pass the base: ");
    let mut base: String = String::new();
    io::stdin()
        .read_line(&mut base)
        .expect("Error in reading base.");
    println!("Pass the answer: ");
    let mut ans: String = String::new();
    io::stdin()
        .read_line(&mut ans)
        .expect("Error in reading answer.");

    println!(
        "The chosen base and answer are {} and {} respectively.",
        base, ans
    );

    let base: i64 = base
        .trim()
        .parse()
        .expect("Unable to parse base input. Please provide valid base.");
    let ans: i64 = ans
        .trim()
        .parse()
        .expect("Unable to parse answer input. Please provide valid base.");

    println!("The answer in decimal is: {}", base_ten_answer(base, ans))
}

fn base_ten_answer(base: i64, mut ans: i64) -> i64 {
    let div = 10;
    let mut po = 0;
    let mut res = 0;
    while ans != 0 {
        let val = ans % div;
        println!("{}", val);
        ans = ans / div;
        res += base.pow(po) * val;
        po += 1;
    }
    res
}
