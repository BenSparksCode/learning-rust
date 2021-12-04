use std::env::{args, Args};

fn main() {
    let mut args: Args = args();

    let first  = args.nth(1).unwrap();
    let operator = args.nth(0).unwrap().chars().next().unwrap();
    let second = args.nth(0).unwrap();

    let first_num = first.parse::<f32>().unwrap();
    let second_num = second.parse::<f32>().unwrap();

    let result = operate(operator, first_num, second_num);

    println!("{:?}", output(first_num, operator, second_num, result));
}

fn operate(operator: char, first_num: f32, second_num: f32) -> f32 {
    if operator == '+' {
        return first_num + second_num
    } else if operator == '-' {
        return first_num - second_num
    } else if operator == '*' {
        return first_num * second_num
    } else if operator == '/' {
        return first_num / second_num
    } else {
        0.0
    }
}

fn output(first_num: f32, operator: char, second_num: f32, result: f32) -> String {
    return format!("{} {} {} = {}", first_num, operator, second_num, result);
}