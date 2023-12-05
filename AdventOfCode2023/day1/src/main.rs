use core::num;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn add_digit_to_number(number: &mut u32, digit: u32) {
    if *number > 9 {
        *number /= 10;
        println!("Removing last digit, now number is {:?}", *number);
    }
    *number = *number * 10 + digit;
    println!("Number is now {:?}", *number);
}

fn get_last_spelled_digit(number_string: &String, digits: &[&str; 9]) -> Option<(u32, i32)> {
    let mut last_digit: u32 = 10;
    let mut last_digit_position: i32 = -1;
    for (i, digit) in digits.iter().enumerate() {
        let maybe_digit = number_string.rfind(digit);
        if let Some(position_digit) = maybe_digit {
            //      println!("Found digit {:?} at pos {:?}", i + 1, position_digit);
            let i32_position_digit: i32 = position_digit.try_into().unwrap();
            if last_digit_position < i32_position_digit {
                last_digit = (i + 1).try_into().unwrap();
                last_digit_position = i32_position_digit;
            }
        }
    }
    if last_digit == 10 {
        None
    } else {
        Some((last_digit, last_digit_position))
    }
}

fn main() {
    let digits = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut sum: u32 = 0;
    for line in reader.lines() {
        let mut number: u32 = 0;
        let mut number_string: String = String::new();
        match line {
            Ok(line) => {
                for (i, ch) in line.chars().enumerate() {
                    println!("Processing {:?}", ch);
                    number_string.push(ch);
                    if ch.is_ascii_digit() {
                        let digit = ch.to_digit(10).unwrap();
                        add_digit_to_number(&mut number, digit);
                    } else {
                        // maybe it's a spelled out number?
                        let maybe_digit = get_last_spelled_digit(&number_string, &digits);
                        if let Some((x, pos)) = maybe_digit {
                            let index: usize = (x - 1).try_into().unwrap();
                            let i_i32: i32 = i.try_into().unwrap();
                            let lungime: i32 = i_i32 - pos + 1;
                            println!(
                                "Lunigme = {:?}, pos = {:?} i_i32 = {:?}",
                                lungime, pos, i_i32
                            );
                            let lungime_usize: usize = lungime.try_into().unwrap();
                            if lungime_usize == digits[index].len() {
                                add_digit_to_number(&mut number, x);
                            }
                        }
                    }
                }
            }
            Err(_) => {
                break;
            }
        }
        if number < 10 {
            number = number * 10 + number;
        }
        println!("Adding number {:?}", number);
        sum += number;
    }
    println!("Result is {:?}", sum);
}
