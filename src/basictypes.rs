
pub fn test01() {
    let x = 5;
    let mut y: u32 = 5;

    assert_eq!(y, 5);
    y = x as u32;

    println!("{}", y);

    let _z = 10;

    println!("Success!");
}


pub fn test02() {
    let _v: u16 = 38_u8 as u16;

    println!("Success!");
}

pub fn test03() {
    let x = 5;
    assert_eq!("i32".to_string(), type_of(&x));

    println!("Success!");
}

fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}


pub fn test04() {
    assert_eq!(i8::MAX, 127);
    assert_eq!(u8::MAX, 255);

    println!("Success!");
}


pub fn test05() {
    let v1 = 251_u8.wrapping_add(8);
    let v2 = i8::checked_add(120, 8).unwrap_or_else(|| {
        println!("Переповнення i8!");
        127
    });
    println!("{},{}", v1, v2);
}



pub fn test06() {
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    assert_eq!(v, 1597);

    println!("Success!");
}



pub fn test07() {
    let x = 1_000.000_1;
    let _y: f32 = 0.12;
    let _z = 0.01_f64;

    assert_eq!(type_of(&x), "f64".to_string());
    println!("Success!");
}


pub fn test08() {

    assert!((0.1f64 + 0.2f64 - 0.3f64).abs() < f64::EPSILON);

    println!("Success!");
}



pub fn test09() {
    let mut sum = 0;
    for i in -3..2 {
        sum += i;
    }

    assert_eq!(sum, -5);

    for c in 'a'..='z' {
        print!("{} ", c as u8);
    }
    println!();
}



pub fn test10() {
    use std::ops::{Range, RangeInclusive};

    assert_eq!(1..5, Range { start: 1, end: 5 });
    assert_eq!(1..=5, RangeInclusive::new(1, 5));

    println!("Success!");
}


pub fn test11() {

    assert_eq!(1u32 + 2, 3);


    assert_eq!(1i32 - 2, -1);
    assert_eq!(1u8.wrapping_sub(2), 255);

    assert_eq!(3 * 50, 150);

    assert!((9.6_f32 / 3.2_f32 - 3.0).abs() < f32::EPSILON);

    assert_eq!(24 % 5, 4);


    assert!(!true == false);
    assert!(true || false);
    assert!(!(true && false));

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
}


