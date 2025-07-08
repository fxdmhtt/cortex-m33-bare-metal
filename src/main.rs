#![no_std]
#![no_main]

extern crate alloc;

// pick a panicking behavior
// use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use alloc::{
    string::{String, ToString},
    vec,
};
use core::cmp::Ordering;
use cortex_m33_bare_metal::{dbg, println};
use cortex_m_rt::entry;
use oorandom::Rand32;

fn hello() {
    println!("Hello, world!");
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn with_in(&self, other: &Rectangle) -> bool {
        self.width < other.width && self.height < other.height
    }
}

impl Rectangle {
    fn new(x: u32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}

#[entry]
fn main() -> ! {
    hello();

    cortex_m33_bare_metal::allocator::init();
    let xs = vec![0, 1, 2];
    println!("{:?}", xs);

    let rect = Rectangle {
        width: 2,
        height: 3,
    };

    dbg!(rect.area());

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    let square = Rectangle::new(10);

    println!("{:?}", square);
    println!("{:#?}", square);
    dbg!(square);

    println!("Can rect1 hold rect2? {}", rect2.with_in(&rect1));
    println!("Can rect1 hold rect3? {}", rect3.with_in(&rect1));

    println!("Guess the number!");

    let mut rng = Rand32::new(42);
    let secret_number = rng.rand_range(1..10);

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        // io::stdin()
        //     .read_line(&mut guess)
        //     .expect("Failed to read line");
        guess += &rng.rand_range(1..10).to_string();

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

    panic!("This is a panic message!\n");

    // loop {
    //     // your code goes here
    // }
}
