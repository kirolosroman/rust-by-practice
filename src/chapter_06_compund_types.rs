#![allow(unused)]
use std::str;
//String
fn ch_6_1_1() {
    let s: &str = "hello, world";

    println!("Success!");
}
fn ch_6_1_2() {
    let s: Box<str> = "hello, world".into();
    greetings(&s)
}
fn greetings(s: &str) {
    println!("{}", s)
}
fn ch_6_1_3() {
    let mut s = String::from("");
    s.push_str("hello, world");
    s.push('!');

    assert_eq!(s, "hello, world!");

    println!("Success!");
}
fn ch_6_1_4() {
    let mut s = String::from("hello");
    s.push(',');
    s.push_str(" world");
    s += "!";

    println!("{}", s);
}
fn ch_6_1_5() {
    let s = String::from("I like dogs");
    // Allocate new memory and store the modified string there
    let s1 = s.replace("dogs", "cats");

    assert_eq!(s1, "I like cats");

    println!("Success!");
}
fn ch_6_1_6() {
    let s = String::from("hello, world");
    greetingsa(s)
}
fn greetingsa(s: String) {
    println!("{}", s)
}
fn ch_6_1_7() {
    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    let s3 = s1.clone() + &s2;
    assert_eq!(s3, "hello,world!");
    println!("{}", s1);
}
fn ch_6_1_8() {
    let s = "hello, world".to_string();
    let s1: &str = &s;

    println!("Success!");
}
fn ch_6_1_9() {
    // You can use escapes to write bytes by their hexadecimal values
    // Fill the blank below to show "I'm writing Rust"
    let byte_escape = "I'm writing Ru\x73t!";
    println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

    // ...Or Unicode code points.
    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

    println!(
        "Unicode character {} (U+211D) is called {}",
        unicode_codepoint, character_name
    );

    let long_string = "String literals
                        can span multiple lines.
                        The linebreak and indentation here \
                         can be escaped too!";
    println!("{}", long_string);
}
fn ch_6_1_10() {
    // Modified: Removed the 'r' to turn it into a normal string literal
    let raw_str = "Escapes don't work here: \x3F \u{211D}";
    assert_eq!(raw_str, "Escapes don't work here: ? ℝ");

    // If you need quotes in a raw string, add a pair of #s
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);

    // If you need "# in your string, just use more #s in the delimiter.
    // You can use up to 65535 #s.
    let delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", delimiter);

    // Fixed: Filled the blank using a raw string with at least three '#' symbols (r###"..."###)
    let long_delimiter = r###"Hello, "##""###;
    assert_eq!(long_delimiter, "Hello, \"##\"");

    println!("Success!");
}
fn ch_6_1_11() {
    let bytestring: &[u8; 21] = b"this is a byte string";
    println!("A byte string: {:?}", bytestring);

    let escaped = b"\x52\x75\x73\x74 as bytes";
    println!("Some escaped bytes: {:?}", escaped);

    let raw_bytestring = br"\u{211D} is not escaped here";
    println!("{:?}", raw_bytestring);

    if let Ok(my_str) = str::from_utf8(raw_bytestring) {
        println!("And the same as text: '{}'", my_str);
    }

    let _quotes = br#"You can also use "fancier" formatting, \
                    like with normal raw strings"#;

    // Byte strings don't have to be UTF-8
    let shift_jis = b"\x82\xe6\x82\xa8\x82\xb1\x82\xb0";

    // FIXED: Added `as &[u8]` so the compiler checks this at runtime instead of compile-time
    match str::from_utf8(shift_jis as &[u8]) {
        Ok(my_str) => println!("Conversion successful: '{}'", my_str),
        Err(e) => println!("Conversion failed: {:?}", e),
    };
}
fn ch_6_1_12() {
    // Fill the blank to print each char in "你好，世界"
    for c in "你好，世界".chars() {
        println!("{}", c)
    }
}
//Array
fn ch_6_2_1() {
    // Fill the blank with proper array type
    let arr: [i32; 5] = [1, 2, 3, 4, 5];

    // Modify the code below to make it work
    assert!(arr.len() == 5);

    println!("Success!");
}
fn ch_6_2_2() {
    // We can ignore parts of the array type or even the whole type, let the compiler infer it for us
    let arr0 = [1, 2, 3];
    let arr: [char; 3] = ['a', 'b', 'c'];

    // Fill the blank
    // Arrays are stack allocated, `std::mem::size_of_val` returns the bytes which an array occupies
    // A char takes 4 bytes in Rust: Unicode char
    assert!(std::mem::size_of_val(&arr) == 12);

    println!("Success!");
}
fn ch_6_2_3() {
    let list: [i32; 100] = [1; 100];

    assert!(list[0] == 1);
    assert!(list.len() == 100);

    println!("Success!");
}
fn ch_6_2_4() {
    // Fix the error
    let _arr = [1, 2, 3];

    println!("Success!");
}
fn ch_6_2_5() {
    let arr = ['a', 'b', 'c'];

    let ele = arr[0]; // Only modify this line to make the code work!

    assert!(ele == 'a');

    println!("Success!");
}
fn ch_6_2_6() {
    let names = [String::from("Sunfei"), "Sunface".to_string()];

    // `Get` returns an Option<&T>, it's safe to use
    let name0 = names.get(0).unwrap();

    // But indexing is not safe
    let _name1 = &names[1];

    println!("Success!");
    let mut x: [i32; 3] = [1, 2, 3];
    x[0] = 10;
    println!("Success!");
}
//Slice
fn ch_6_3_1() {
    let arr = [1, 2, 3];
    let s1: &[i32] = &arr[0..2];

    let s2: &str = "hello, world" as &str;

    println!("Success!");
}
fn ch_6_3_2() {
    let arr: [char; 3] = ['中', '国', '人'];

    let slice: &[char] = &arr[..2];

    // Modify '8' to make it work
    // TIPS: slice( reference ) IS NOT an array, if it is an array, then `assert!` will be passed: Each of the two chars '中' and '国'  occupies 4 bytes, 2 * 4 = 8
    assert!(std::mem::size_of_val(&slice) == 16);

    println!("Success!");
}
fn ch_6_3_3() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    // Fill the blanks to make the code work
    let slice: &[i32] = &arr[1..4];
    assert_eq!(slice, &[2, 3, 4]);

    println!("Success!");
}
fn ch_6_3_4() {
    let s = String::from("hello");

    let slice1 = &s[0..2];
    // Fill the blank to make the code work, DON'T USE 0..2 again
    let slice2 = &s[..2];

    assert_eq!(slice1, slice2);

    println!("Success!");
}
fn ch_6_3_5() {
    let s = "你好，世界";
    // Modify this line to make the code work
    let slice = &s[0..3];

    assert!(slice == "你");

    println!("Success!");
}
fn ch_6_3_6() {
    // Fix errors
    fn main() {
        let mut s = String::from("hello world");

        // Here, &s is `&String` type, but `first_letter` needs a `&str` type.
        // It works because `&String` can be implicitly converted to `&str. If you want to know more, this is called `Deref coercion`.
        let letter = first_letter(&s);

        println!("the first letter is: {}", letter);
        s.clear(); // error!
    }
}
fn first_letter(s: &str) -> &str {
    &s[..1]
}
//Tuple
fn ch_6_4_1() {
    let _t0: (u8, i16) = (0, -1);
    // Tuples can be tuple's members
    let _t1: (u8, (i16, u32)) = (0, (-1, 1));
    // Fill the blanks to make the code work
    let t: (u8, u16, i64, &str, String) = (1u8, 2u16, 3i64, "hello", String::from(", world"));

    println!("Success!");
}
fn ch_6_4_2() {
    let t = ("i", "am", "sunface");
    assert_eq!(t.2, "sunface");

    println!("Success!");
}
fn ch_6_4_3() {
    let too_long_tuple = ((1, 2, 3, 4, 5, 6), (7, 8, 9, 10, 11, 12, 13));
    println!("too long tuple: {:?}", too_long_tuple);
}
fn ch_6_4_4() {
    let tup = (1, 6.4, "hello");

    // Fill the blank to make the code work
    let (x, z, y) = tup;

    assert_eq!(x, 1);
    assert_eq!(y, "hello");
    assert_eq!(z, 6.4);

    println!("Success!");
}
fn ch_6_4_5() {
    let (x, y, z);

    // Fill the blank
    (y, z, x) = (1, 2, 3);

    assert_eq!(x, 3);
    assert_eq!(y, 1);
    assert_eq!(z, 2);

    println!("Success!");
}
fn ch_6_4_6() {
    // Fill the blank, need a few computations here.
    let (x, y) = sum_multiply((2, 3));

    assert_eq!(x, 5);
    assert_eq!(y, 6);

    println!("Success!");
}
fn sum_multiply(nums: (i32, i32)) -> (i32, i32) {
    (nums.0 + nums.1, nums.0 * nums.1)
}
//Struct
struct Person {
    name: String,
    age: u8,
    hobby: String,
}
fn ch_6_5_1() {
    let age = 30;
    let p = Person {
        name: String::from("sunface"),
        age,
        hobby: String::from("football"),
    };

    println!("Success!");
}
fn ch_6_5_2() {}
fn ch_6_5_3() {}
fn ch_6_5_4() {}
fn ch_6_5_5() {}
fn ch_6_5_6() {}
fn ch_6_5_7() {}
fn ch_6_5_8() {}

pub fn calling_main() {
    //String
    ch_6_1_1();
    ch_6_1_2();
    ch_6_1_3();
    ch_6_1_4();
    ch_6_1_5();
    ch_6_1_6();
    ch_6_1_7();
    ch_6_1_8();
    ch_6_1_9();
    ch_6_1_10();
    ch_6_1_11();
    ch_6_1_12();
    //Array
    ch_6_2_1();
    ch_6_2_2();
    ch_6_2_3();
    ch_6_2_4();
    ch_6_2_5();
    ch_6_2_6();
    //Slice
    ch_6_3_1();
    ch_6_3_2();
    ch_6_3_3();
    ch_6_3_4();
    ch_6_3_5();
    ch_6_3_6();
    //tuple
    ch_6_4_1();
    ch_6_4_2();
    ch_6_4_3();
    ch_6_4_4();
    ch_6_4_5();
    ch_6_4_6();
    //struct
    ch_6_5_1();
    ch_6_5_2();
    ch_6_5_3();
    ch_6_5_4();
    ch_6_5_5();
    ch_6_5_6();
    ch_6_5_7();
    ch_6_5_8();
}
