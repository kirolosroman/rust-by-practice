#![allow(unused)]

mod flow_control {
    pub fn ch_07_01() {
        let n = 5;

        if n < 0 {
            println!("{} is negative", n);
        } else if n > 0 {
            println!("{} is positive", n);
        } else {
            println!("{} is zero", n);
        }
    }
    pub fn ch_07_02() {
        let n = 5;

        let big_n = if n < 10 && n > -10 {
            println!(", and is a small number, increase ten-fold");

            10 * n
        } else {
            println!(", and is a big number, halve the number");

            n / 2.0 as i32
        };

        println!("{} -> {}", n, big_n);
    }
    pub fn ch_07_03() {
        for n in 1..=99 {
            // modify this line to make the code work
            if n == 100 {
                panic!("NEVER LET THIS RUN")
            }
        }

        println!("Success!");
    }
    pub fn ch_07_04() {
        let names = [String::from("liming"), String::from("hanmeimei")];
        for name in &names {
            // Do something with name...
        }

        println!("{:?}", names);

        let numbers = [1, 2, 3];
        // The elements in numbers are Copy，so there is no move here
        for n in numbers {
            // Do something with n...
        }

        println!("{:?}", numbers);
    }
    pub fn ch_07_05() {
        let a = [4, 3, 2, 1];

        // Iterate the indexing and value in 'a'
        for (i, v) in a.iter().enumerate() {
            println!("The {}th element is {}", i + 1, v);
        }
    }
    pub fn ch_07_06() {
        // A counter variable
        let mut n = 1;

        // Loop while the condition is true
        while n <= 10 {
            if n % 15 == 0 {
                println!("fizzbuzz");
            } else if n % 3 == 0 {
                println!("fizz");
            } else if n % 5 == 0 {
                println!("buzz");
            } else {
                println!("{}", n);
            }

            n = n + 1;
        }

        println!("n reached {}, so loop is over", n);
    }
    pub fn ch_07_07() {
        let mut n = 0;
        for i in 0..=100 {
            if n == 66 {
                break;
            }
            n += 1;
        }

        assert_eq!(n, 66);

        println!("Success!");
    }
    pub fn ch_07_08() {
        let mut n = 0;
        for i in 0..=100 {
            if n != 66 {
                n += 1;
                continue;
            }

            continue;
        }

        assert_eq!(n, 66);

        println!("Success!");
    }
    pub fn ch_07_09() {
        let mut count = 0u32;

        println!("Let's count until infinity!");

        // Infinite loop
        loop {
            count += 1;

            if count == 3 {
                println!("three");

                // Skip the rest of this iteration
                continue;
            }

            println!("{}", count);

            if count == 5 {
                println!("OK, that's enough");

                break;
            }
        }

        assert_eq!(count, 5);

        println!("Success!");
    }
    pub fn ch_07_10() {
        let mut counter = 0;

        let result = loop {
            counter += 1;

            if counter == 10 {
                break counter * 2;
            }
        };

        assert_eq!(result, 20);

        println!("Success!");
    }
    pub fn ch_07_11() {
        let mut count = 0;
        'outer: loop {
            'inner1: loop {
                if count >= 20 {
                    // This would break only the inner1 loop
                    break 'inner1; // `break` is also works.
                }
                count += 2;
            }

            count += 5;

            'inner2: loop {
                if count >= 30 {
                    // This breaks the outer loop
                    break 'outer;
                }

                // This will continue the outer loop
                continue 'outer;
            }
        }

        assert!(count == 30);

        println!("Success!");
    }
}
pub fn calling_main() {
    println!("\n=== Flow Control EXAMPLES ===\n");
    flow_control::ch_07_01();
    flow_control::ch_07_02();
    flow_control::ch_07_03();
    flow_control::ch_07_04();
    flow_control::ch_07_05();
    flow_control::ch_07_06();
    flow_control::ch_07_07();
    flow_control::ch_07_08();
    flow_control::ch_07_09();
    flow_control::ch_07_10();
    flow_control::ch_07_11();
}
