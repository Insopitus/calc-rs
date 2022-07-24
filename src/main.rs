use calc_rs::{Calculator, Error};

fn main() {
    println!("A CLI calculator.");
    loop {
        println!("Input your express:");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        match Calculator::parse(&input) {
            Ok(result) => {
                println!("The result is: {}", result);
            }
            Err(err) => match err {
                Error::BadToken(t) => {
                    println!("Bad token: {}", t);
                }
                Error::MismatchedParens => {
                    println!("Parentheses don't match!")
                }
            },
        }
    }
}
