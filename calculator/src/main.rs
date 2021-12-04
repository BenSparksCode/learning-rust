use std::env::{args, Args};

fn main() {
    let mut args: Args = args();

    let first  = args.nth(1).unwrap();
    let operator = args.nth(0).unwrap();
    let second = args.nth(0).unwrap();

    let first_num = first.parse::<f32>().unwrap();
    let second_num = second.parse::<f32>().unwrap();

    println!("{:?} {} {}", first, operator, second);
}
