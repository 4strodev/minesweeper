use std::io::{self, Write};

pub struct Prompt<'a> {
    prompt_symbol: &'a str,
}

impl Prompt<'_> {
    pub fn new(symbol: &str) -> Prompt {
        return Prompt {
            prompt_symbol: symbol,
        };
    }

    pub fn input_as<T: std::str::FromStr>(&self, message: &str) -> T
    where
        <T as std::str::FromStr>::Err: std::fmt::Display,
    {
        let value: T;
        loop {
            let guess = self.input(message);

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

    fn input(&self, message: &str) -> String {
        let mut guess: String = String::new();
        print!("{}\n{} ", message, self.prompt_symbol);
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
}
