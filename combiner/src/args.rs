fn get_nth_arg(n: usize) -> String {
    return std::env::args().nth(n).unwrap();
}

