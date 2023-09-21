use std::fmt;

struct Circle {
    radius: i32
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circle of radius {}", self.radius)
    }
}

fn converting_to_string() {
    println!("");
    println!("start converting_to_string");

    let circle = Circle { radius: 6 };
    println!("{}", circle.to_string());

    println!("end converting_to_string");
    println!("");
}

fn parsing_a_string() {
    println!("");
    println!("start parsing_a_string");

    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println!("Sum: {:?}", sum);

    println!("end parsing_a_string");
    println!("");
}

use std::convert::TryFrom;
use std::convert::TryInto;

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
}

fn tryfrom_tryinto() {
    println!("");
    println!("start tryfrom_tryinto");

    // TryFrom

    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));

    // TryInto

    let result: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));
    let result: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));

    println!("end tryfrom_tryinto");
    println!("");
}

use std::convert::From;

#[derive(Debug)]
#[allow(dead_code)] // `#[warn(dead_code)]` on by default
struct NumberF {
    value: i32,
}

#[derive(Debug)]
#[allow(dead_code)] // `#[warn(dead_code)]` on by default
struct NumberI {
    value: i32,
}

impl From<i32> for NumberF {
    fn from(item: i32) -> Self {
        NumberF { value: item }
    }
}

use std::convert::Into;

impl Into<NumberI> for i32 {
    fn into(self) -> NumberI {
        NumberI { value: self }
    }
}

fn test_into() {
    println!("");
    println!("start test_into");

    let int = 5;
    // Try removing the type annotation
    let num: NumberI = int.into();
    println!("My number is {:?}", num);

    println!("end test_into");
    println!("");
}

fn test_from() {
    println!("");
    println!("start test_from");

    let num = NumberF::from(30);
    println!("My number is {:?}", num);

    println!("end test_from");
    println!("");
}

fn main() {
    test_from();
    test_into();
    tryfrom_tryinto();
    converting_to_string();
    parsing_a_string();
}
