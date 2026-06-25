#![allow(unused)]
mod matchs {
    enum Direction {
        East,
        West,
        North,
        South,
    }
    pub fn ch_08_01_01() {
        let dire = Direction::East;
        match dire {
            Direction::East => println!("East"),
            Direction::North | Direction::South => println!("North or South"),
            Direction::West => println!("West"),
        }
    }
    pub fn ch_08_01_02() {
        let boolean = true;

        // Fill the blank with a match expression:
        //
        // boolean = true => binary = 1
        // boolean = false =>  binary = 0
        let binary = match boolean {
            true => 1,
            false => 0,
        };

        assert_eq!(binary, 1);

        println!("Success!");
    }
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    fn show_message(msg: Message) {
        match msg {
            Message::Move { x: a, y: b } => {
                // match  Message::Move
                assert_eq!(a, 1);
                assert_eq!(b, 3);
            }
            Message::ChangeColor(_, g, b) => {
                assert_eq!(g, 255);
                assert_eq!(b, 0);
            }
            __ => println!("no data in these variants"),
        }
    }
    pub fn ch_08_01_03() {
        let msgs = [
            Message::Quit,
            Message::Move { x: 1, y: 3 },
            Message::ChangeColor(255, 255, 0),
        ];

        for msg in msgs {
            show_message(msg)
        }

        println!("Success!");
    }
    //matches
    pub fn ch_08_01_04() {
        let alphabets = ['a', 'E', 'Z', '0', 'x', '9', 'Y'];

        // Fill the blank with `matches!` to make the code work
        for ab in alphabets {
            assert!(matches!(ab,'a'..='z'|'A'..='Z'|'0'..='9'))
        }

        println!("Success!");
    }
    enum MyEnum {
        Foo,
        Bar,
    }

    pub fn ch_08_01_05() {
        let mut count = 0;

        let v = vec![MyEnum::Foo, MyEnum::Bar, MyEnum::Foo];
        for e in v {
            if matches!(e, MyEnum::Foo) {
                // Fix the error by changing only this line
                count += 1;
            }
        }

        assert_eq!(count, 2);

        println!("Success!");
    }
    //If let
    pub fn ch_08_01_06() {
        let o = Some(7);

        // Remove the whole `match` block, using `if let` instead
        if let Some(i) = o {
            println!("This is a really long string and `{:?}`", i);

            println!("Success!");
        }
    }
    enum Foo {
        Bar(u8),
        Far(u8),
    }

    pub fn ch_08_01_07() {
        let a = Foo::Bar(2);

        if let Foo::Bar(i) = a {
            println!("foobar holds the value: {}", i);

            println!("Success!");
        }
    }
    enum Foos {
        Bar,
        Baz,
        Qux(u32),
    }
    pub fn ch_08_01_08() {
        let a = Foos::Qux(10);

        match a {
            Foos::Bar => println!("match foo::bar"),
            Foos::Baz => println!("match foo::baz"),
            _ => println!("match others"),
        }
    }
    pub fn ch_08_01_09() {
        let age = Some(30);
        if let Some(age) = age {
            // Create a new variable with the same name as previous `age`
            assert_eq!(age, 30);
        } // The new variable `age` goes out of scope here

        match age {
            // Match can also introduce a new shadowed variable
            Some(age) => println!("age is a new variable, it's value is {}", age),
            _ => (),
        }
    }
}
mod patterns {
    fn match_number(n: i32) {
        match n {
            // Match a single value
            1 => println!("One!"),
            // Fill in the blank with `|`, DON'T use `..` or `..=`
            2 | 3 | 4 | 5 => println!("match 2 -> 5"),
            // Match an inclusive range
            6..=10 => {
                println!("match 6 -> 10")
            }
            _ => {
                println!("match -infinite -> 0 or 11 -> +infinite")
            }
        }
    }
    pub fn ch_08_02_01() {
        match_number(3);
    }
    struct Point {
        x: i32,
        y: i32,
    }
    pub fn ch_08_02_02() {
        // Fill in the blank to let p match the second arm
        let p = Point { x: 0, y: 10 };

        match p {
            Point { x, y: 0 } => println!("On the x axis at {}", x),
            // Second arm
            Point {
                x: 0..=5,
                y: y @ (10 | 20 | 30),
            } => println!("On the y axis at {}", y),
            Point { x, y } => println!("On neither axis: ({}, {})", x, y),
        }
    }
    enum Message {
        Hello { id: i32 },
    }
    pub fn ch_08_02_03() {
        let msg = Message::Hello { id: 5 };

        match msg {
            Message::Hello { id: id @ 3..=7 } => println!("Found an id in range [3, 7]: {}", id),
            Message::Hello {
                id: newid @ (10 | 11 | 12),
            } => {
                println!("Found an id in another range [10, 12]: {}", newid)
            }
            Message::Hello { id } => println!("Found some other id: {}", id),
        }
    }
    pub fn ch_08_02_04() {
        let num = Some(4);
        let split = 5;
        match num {
            Some(x) if x < split => assert!(x < split),
            Some(x) => assert!(x >= split),
            None => (),
        }

        println!("Success!");
    }
    pub fn ch_08_02_05() {
        let numbers = (2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048);

        match numbers {
            (first, .., last) => {
                assert_eq!(first, 2);
                assert_eq!(last, 2048);
            }
        }

        println!("Success!");
    }
    pub fn ch_08_02_06() {
        let mut v = String::from("hello,");
        let r = &mut v;

        match r {
            value => value.push_str(" world!"),
        }
    }
}
pub fn calling_main() {
    println!("\n=== Pattern Match EXAMPLES ===\n");
    matchs::ch_08_01_01();
    matchs::ch_08_01_02();
    matchs::ch_08_01_03();
    matchs::ch_08_01_04();
    matchs::ch_08_01_05();
    matchs::ch_08_01_06();
    matchs::ch_08_01_07();
    matchs::ch_08_01_08();
    matchs::ch_08_01_09();
    patterns::ch_08_02_01();
    patterns::ch_08_02_02();
    patterns::ch_08_02_03();
    patterns::ch_08_02_04();
    patterns::ch_08_02_05();
    patterns::ch_08_02_06();
}
