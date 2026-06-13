#![allow(unused)]
use std::mem::size_of_val;
use std::ops::{Range, RangeInclusive};

// Numbers
fn ch_4_1_1() {
    let x: i32 = 5;
    let mut y: i32 = 5;

    y = x;

    let z: i32 = 10; // Type of z ? 

    println!("1- Success!");
}
fn ch_4_1_2() {
    let v: u16 = 38_u8 as u16;

    println!("2- Success!");
}
fn ch_4_1_3() {
    let x = 5;
    assert_eq!("i32".to_string(), type_of(&x));

    println!("3- Success!");
}
fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}
fn ch_4_1_4() {
    assert_eq!(i8::MAX, 127);
    assert_eq!(u8::MAX, 255);

    println!("4- Success!");
}
fn ch_4_1_5() {
    let v1 = 120_u8 + 8;
    let v2 = i8::checked_add(119, 8).unwrap();
    println!("5- Success! {},{}", v1, v2);
}
fn ch_4_1_6() {
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    //println!("{}",v);
    assert!(v == 1597);

    println!("6- Success!");
}
fn ch_4_1_7() {
    let x: f32 = 1_000.000_1; // ?
    let y: f32 = 0.12; // f32
    let z = 0.01_f64; // f64

    assert_eq!(type_of_j(&x), "f32".to_string());
    println!("7- Success! ");
}
fn type_of_j<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}
fn ch_4_1_8() {
    assert!(0.1_f32 + 0.2_f32 == 0.3_f32);

    println!("8- Success!");
}
fn ch_4_1_9() {
    let mut sum = 0;
    for i in -3..2 {
        sum += i
    }

    assert!(sum == -5);

    for c in 'a'..='z' {
        println!("{}", c as u8);
    }
}
fn ch_4_1_10() {
    assert_eq!((1..5), Range { start: 1, end: 5 });
    assert_eq!((1..=5), RangeInclusive::new(1, 5));

    println!("10- Success!");
}
fn ch_4_1_11() {
    // Integer addition
    assert!(1u32 + 2u32 == 3u32);

    // Integer subtraction
    assert!(1i32 - 2i32 == -1i32);
    assert!(1u8 + 2u8 == 3u8);

    assert!(3 * 50 == 150);

    assert!(9.6f32 / 3.2f32 == 3.0f32); // error ! make it work

    assert!(24 % 5 == 4);
    // Short-circuiting boolean logic
    assert!(true && false == false);
    assert!(true || false == true);
    assert!(!true == false);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
}
//char, bool, unit
fn ch4_2_1() {
    let c1 = 'a';
    assert_eq!(size_of_val(&c1), 4);

    let c2 = '中';
    assert_eq!(size_of_val(&c2), 4);

    println!("Success!");
}
fn ch4_2_2() {
    let c1 = '中';
    print_char(c1);
}
fn print_char(c: char) {
    println!("{}", c);
}
fn ch4_2_3() {
    let _f: bool = false;

    let t = true;
    if t {
        println!("Success!");
    }
}
fn ch4_2_4() {
    let f = false;
    let t = true && false;
    assert_eq!(t, f);

    println!("Success!");
}
fn ch4_2_5() {
    let _v: () = ();

    let v = (2, 3);
    assert_eq!(_v, implicitly_ret_unit());

    println!("Success!");
}
fn implicitly_ret_unit() {
    println!("I will return a ()");
}

// Don't use this one
fn explicitly_ret_unit() -> () {
    println!("I will return a ()");
}
fn ch4_2_6() {
    let unit: () = ();
    assert!(size_of_val(&unit) == 0);

    println!("Success!");
}
//Statements and Expressions
fn ch_4_3_1() {
    let v = {
        let mut x = 1;
        x + 2
    };

    assert_eq!(v, 3);

    println!("Success!");
}
fn ch_4_3_2() {
    let v = {
        let x = 3;
        x
    };

    assert!(v == 3);

    println!("Success!");
}
fn ch_4_3_3() {
    let s = sum(1, 2);
    assert_eq!(s, 3);

    println!("Success!");
}
fn sum(x: i32, y: i32) -> i32 {
    x + y
}
//Functions
fn ch_4_4_1() {
    print();
}
fn print() -> () {
    println!("Success!");
}
pub fn calling_main() {
    ch_4_1_1();
    ch_4_1_2();
    ch_4_1_3();
    ch_4_1_4();
    ch_4_1_5();
    ch_4_1_6();
    ch_4_1_7();
    ch_4_1_8();
    ch_4_1_9();
    ch_4_1_10();
    ch_4_1_11();
    ch4_2_1();
    ch4_2_2();
    ch4_2_3();
    ch4_2_4();
    ch4_2_5();
    ch4_2_6();
    ch_4_3_1();
    ch_4_3_2();
    ch_4_3_3();
    ch_4_4_1();
}
