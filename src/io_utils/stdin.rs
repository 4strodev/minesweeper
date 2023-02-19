use std::io::{self, Write};

pub fn input_as<T: std::str::FromStr>(message: &str) -> T
where
    <T as std::str::FromStr>::Err: std::fmt::Display,
{
    let value: T;
    loop {
        let guess = input(message);

        match guess.parse::<T>() {
            Ok(val) => value = val,
            Err(err) => {
                println!("Error converting from str: {}", err);
                continue;
            }
        }
        break;
    }

    return value;
}

fn input(message: &str) -> String {
    let mut guess: String = String::new();
    print!("{}{}", message, crate::PROMPT);
    match io::stdout().flush() {
        Ok(()) => (),
        Err(err) => panic!("Error flushing stdout! {}", err),
    };

    match std::io::stdin().read_line(&mut guess) {
        Ok(_) => (),
        Err(err) => panic!("Error reading from stdin: {}", err),
    };

    return guess.trim().to_string();
}

