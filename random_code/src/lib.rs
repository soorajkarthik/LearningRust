pub mod shapes {
    #[derive(Debug)]//basically toString functionality
    pub struct Rectangle<'a, 'b> {
        width: &'a i32,
        height: &'b i32,
    }

    impl<'a, 'b> Rectangle<'a, 'b> {
        pub fn area(&self) -> i32 {
            self.width * self.height
        }

        pub fn from(width: &'a i32, height: &'b i32) -> Rectangle<'a, 'b> {
            Rectangle {
                width,
                height,
            }
        }

        pub fn square(size: &i32) -> Rectangle {
            Rectangle {
                width: size,
                height: size,
            }
        }
    }
}

pub mod functions {
    use std::cmp::Ordering;
    use std::fs::read;
    use std::io;
    use std::io::Write;

    use chrono::{Datelike, Utc};
    use rand::Rng;

    pub fn var_fibonacci(n: i32) -> i32 {
        if n == 0 || n == 1 {
            return 1;
        } else {
            let mut a = 0;
            let mut b = 1;
            let mut c;

            for _i in 1..n {
                c = a + b;
                a = b;
                b = c;
            }
            return b;
        }
    }

    pub fn arr_fibonacci(n: i32) -> i32 {
        if n == 0 || n == 1 {
            return 1;
        } else {
            let mut arr = vec![1; n as usize];
            for i in 2..n {
                let index = i as usize;
                arr[index] = arr[index - 1] + arr[index - 2];
            }
            return arr[n as usize - 1];
        }
    }

    pub fn math_fibonacci(n: i32) -> u64 {
        let phi = (1f64 + (5f64).sqrt()) / (2f64);
        return (phi.powi(n) / (5f64).sqrt()).floor() as u64;
    }

    pub fn guessing_game() {
        let secret_number = rand::thread_rng().gen_range(1, 101);
        println!("Guess the number!");

        loop {
            println!("Please input your guess");
            let mut guess = String::new();

            io::stdin().read_line(&mut guess)
                .expect("Failed to read line");

            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue
            };

            println!("You guessed {}", guess);
            match guess.cmp(&secret_number) {
                Ordering::Less => println!("Too small!"),
                Ordering::Equal => {
                    println!("You win!");
                    break;
                }
                Ordering::Greater => println!("Too big!")
            }
        }
    }

    pub fn year_you_turn_100() {
        const ERROR_MESSAGE: &str = "Something went wrong :(";

        let name = input("What is your name?")
            .expect(ERROR_MESSAGE);
        let age: i32 = input("How old are you?")
            .expect(ERROR_MESSAGE)
            .parse()
            .expect("Age invalid");
        let curr_year = Utc::now().year();
        let res = 100 - age + curr_year;
        if age > 100 {
            println!("{} turned 100 in {}", name, res);
        } else {
            println!("{} will turn 100 in {}", name, res);
        }
    }

    pub fn first_word(s: &str) -> &str {
        for (i, &letter) in s.as_bytes().iter().enumerate() {
            if letter == b' ' {
                return &s[0..i];
            }
        }

        return &s;
    }

    pub fn input(message: &str) -> io::Result<String> {
        print!("{} ", message);
        io::stdout().flush()?;

        let mut buffer: String = String::new();
        io::stdin().read_line(&mut buffer)?;
        return Ok(buffer.trim().to_owned());
    }

    pub fn to_pig_latin(input: &str) -> String {
        let mut res = String::new();
        for word in input.split_whitespace() {
            if "aeiouAEIOU".contains(&word[0..1]) {
                res = format!("{} {}-{}", res, word, "hay");
            } else {
                res = format!("{} {}{}{}", res, &word[1..], &word[0..1], "ay");
            }
        }

        return res.trim().to_owned();
    }

    pub fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
        let mut largest = list[0];
        for &element in &list[1..] {
            if element > largest {
                largest = element;
            }
        }
        largest
    }
}


