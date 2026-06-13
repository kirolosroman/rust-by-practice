#![allow(unused)]

//Owenership
fn ch_5_1_1() {
    // Use as many approaches as you can to make it work
    let x = String::from("Hello world");
    let y = &x;
    println!("{}, {}", x, y);
}
fn ch_5_1_2() {
    let s1 = String::from("Hello world");
    let s2 = take_ownership(s1);

    println!("{}", s2);
}
fn take_ownership(s: String) -> String {
    println!("{}", s);
    s
}
fn ch_5_1_3() {
    let s = give_ownership();
    println!("{}", s);
}
fn give_ownership() -> String {
    let s = String::from("Hello world");
    // Convert String to Vec
    let _s = s.clone().into_bytes();
    s
}
fn ch_5_1_4() {
    let s = String::from("Hello World");

    let s = print_str(s);

    println!("{}", s);
}
fn print_str(s: String) -> String {
    println!("{}", s);
    s
}

fn ch_5_1_5() {
    let x = (1, 2, (), "hello");
    let y = x;
    println!("{:?}, {:?}", x, y);
}
fn ch_5_1_6() {
    let s = String::from("Hello ");

    let mut s1 = s;

    s1.push_str("World!");

    println!("Success!");
}
fn ch_5_1_7() {
    let x = Box::new(5);
    let mut y = Box::new(*x); // update this line, don't change other lines!
    *y = 4;
    assert_eq!(*y, 4);

    println!("Success!");
}
fn ch_5_1_8() {
    let t = (
        String::from("hello"),
        String::from("world"),
        String::from("!"),
    );

    let _s = t.0;

    // Modify this line only, don't use `_s`
    println!("{:?}", (t.1, t.2));
}
fn ch_5_1_9() {
    let t = (String::from("hello"), String::from("world"));
    // Fill the blanks
    let (s1, s2) = (&t.0, &t.1);
    println!("{:?}, {:?}, {:?}", s1, s2, t); // -> "hello", "world", ("hello", "world")
}

//Reference and Borrowing
fn ch_5_2_1() {
    let x = 5;
    // Fill the blank
    let p = &x;

    println!("the memory address of x is {:p}", p); // One possible output: 0x16fa3ac84
}
fn ch_5_2_2() {
    let x = 5;
    let y = &x;

    // Modify this line only
    assert_eq!(5, *y);

    println!("Success!");
}
fn ch_5_2_3() {
    let mut s = String::from("hello, ");

    borrow_object(&s);

    println!("Success!");
}
fn borrow_object(s: &String) {}
fn ch_5_2_4() {
    let mut s = String::from("hello, ");

    push_str(&mut s);

    println!("{}", s);
}
fn push_str(s: &mut String) {
    s.push_str("world")
}
fn ch_5_2_5() {
    let mut s = String::from("hello, ");

    // Fill the blank to make it work
    let p = &mut s;

    p.push_str("world");

    println!("Success! {}", s);
}
fn ch_5_2_6() {
    let c = '中';

    let r1 = &c;
    // Fill the blank，dont change other code
    let ref r2 = c;

    assert_eq!(*r1, *r2);

    // Check the equality of the two address strings
    assert_eq!(get_addr(r1), get_addr(r2));

    println!("Success!");
}
// Get memory address string
fn get_addr(r: &char) -> String {
    format!("{:p}", r)
}
// mutable variable can have only one mutable reference at a time, but it can have any number of immutable references
fn ch_5_2_7() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;

    println!("{}, {}", r1, r2);

    println!("Success!");
}
fn ch_5_2_8() {
    // Fix error by modifying this line
    let mut s = String::from("hello, ");

    borrow_object(&mut s);

    println!("Success!");
}
fn borrow_object_1(s: &mut String) {}
fn ch_5_2_9() {
    let mut s = String::from("hello, ");

    borrow_object_2(&mut s);

    s.push_str("world");

    println!("Success!");
}
fn borrow_object_2(s: &mut String) {
    s.push_str("jkdf");
}
//Non-Lexical Lifetimes (NLL).
fn ch_5_2_10() {
    let mut s = String::from("hello, ");

    let r1 = &mut s;
    r1.push_str("world");
    let r2 = &mut s;
    r2.push_str("!");

    //println!("{}",r1);
}
fn ch_5_2_11() {
    let mut s = String::from("hello, ");

    let r1 = &mut s;
    let r2 = &mut s;

    // Add one line below to make a compiler error: cannot borrow `s` as mutable more than once at a time
    // You can't use r1 and r2 at the same time
    //println!("{}", r1);
}

pub fn calling_main() {
    ch_5_1_1();
    ch_5_1_2();
    ch_5_1_3();
    ch_5_1_4();
    ch_5_1_5();
    ch_5_1_6();
    ch_5_1_7();
    ch_5_1_8();
    ch_5_1_9();
    /////////
    ch_5_2_1();
    ch_5_2_2();
    ch_5_2_3();
    ch_5_2_4();
    ch_5_2_5();
    ch_5_2_6();
    ch_5_2_7();
    ch_5_2_8();
    ch_5_2_9();
    ch_5_2_10();
    ch_5_2_11();
}
