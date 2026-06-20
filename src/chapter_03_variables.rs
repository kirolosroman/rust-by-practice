#![allow(unused)]
fn ch_3_01() {
    let x: i32 = 5; // Uninitialized but used, ERROR !
    let y: i32 = 5; // Uninitialized but also unused, only a Warning !
    assert_eq!(x, y);
    println!("Success!");
}
fn ch_3_02() {
    let mut x: i32 = 1;
    x += 2;

    assert_eq!(x, 3);
    println!("Success!");
}

//Scope
//A scope is the range within the program for which the item is valid.
fn ch_3_03() {
    let x: i32 = 10;
    {
        let y: i32 = 5;
        println!("Inner scope value of x is {} and value of y is {}", x, y);
    }
    println!("Outer scope value of x is {}", x);
}
fn ch_3_04() {
    let x = define_x();
    println!("{}, world", x);
}
fn define_x() -> &'static str {
    let x: &str = "hello";
    x
}

//Shadowing
//You can declare a new variable with the same name as a previous variable,
//here we can say the first one is shadowed by the second one.
fn ch_3_05() {
    let x: i32 = 5;
    {
        let x = 12;
        assert_eq!(x, 12);
    }

    assert_eq!(x, 5);

    let x = 42;
    println!("{}", x); // Prints "42".
}
fn ch_3_06() {
    let mut x: i32 = 1;
    // Shadowing and re-binding
    x = 7;
    x += 3;

    let y = 4;
    // Shadowing
    let y = "I can also be bound to text!";

    println!("Success!");
}
fn ch_3_07() {
    let x = 1;
    println!("The value of x is: {}", x);
}
fn ch_3_08() {
    let (mut x, y) = (1, 2);
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);

    println!("Success!");
}
fn ch_3_09() {
    let (x, y);
    (x, ..) = (3, 4);
    [_, y] = [1, 2];
    // Fill the blank to make the code work
    assert_eq!([x, y], [3, 2]);

    println!("Success!");
}
pub fn calling_main() {
    ch_3_01();
    ch_3_02();
    ch_3_03();
    ch_3_04();
    ch_3_05();
    ch_3_06();
    ch_3_07();
    ch_3_08();
    ch_3_09();
}
