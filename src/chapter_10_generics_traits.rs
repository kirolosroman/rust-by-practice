#![allow(unused)]
mod generics {
    use std::fmt::{self, Debug, Formatter};
    use std::ops::Add;

    struct A; // Concrete type `A`.
    struct S(A); // Concrete type `S`.
    struct SGen<T>(T); // Generic type `SGen`.

    fn reg_fn(_s: S) {}

    fn gen_spec_t(_s: SGen<A>) {}

    fn gen_spec_i32(_s: SGen<i32>) {}

    fn generic<T>(_s: SGen<T>) {}

    pub fn ch10_01_01() {
        // Using the non-generic functions
        reg_fn(S(A)); // Concrete type.
        gen_spec_t(SGen(A)); // Implicitly specified type parameter `A`.
        gen_spec_i32(SGen(42)); // Implicitly specified type parameter `i32`.

        // Explicitly specified type parameter `char` to `generic()`.
        generic::<char>(SGen('z'));

        // Implicitly specified type parameter `char` to `generic()`.
        generic(SGen('a'));

        println!("Success!");
    }
    fn sum<T: Add<Output = T>>(a: T, b: T) -> T {
        a + b
    }
    pub fn ch10_01_02() {
        assert_eq!(5, sum(2i8, 3i8));
        assert_eq!(50, sum(20, 30));
        assert_eq!(2.46, sum(1.23, 1.23));

        println!("Success!");
    }
    struct Point<T> {
        x: T,
        y: T,
    }
    pub fn ch10_01_03() {
        let integer = Point { x: 5, y: 10 };
        let float = Point { x: 1.0, y: 4.0 };

        println!("Success!");
    }
    struct Points<T, U> {
        x: T,
        y: U,
    }
    pub fn ch10_01_04() {
        // DON'T modify this code.
        let p = Points {
            x: 5,
            y: "hello".to_string(),
        };

        println!("Success!");
    }
    struct Val<T> {
        val: T,
    }

    // Make the implementation block generic as well
    impl<T> Val<T> {
        // Change the return type to match the generic reference &T
        fn value(&self) -> &T {
            &self.val
        }
    }
    pub fn ch10_01_05() {
        // DON'T modify the code in `main`.
        let x = Val { val: 3.0 };
        let y = Val {
            val: "hello".to_string(),
        };
        println!("{}, {}", x.value(), y.value());
    }
    struct Pointss<T, U> {
        x: T,
        y: U,
    }

    impl<T, U> Pointss<T, U> {
        // Implement mixup to make it work, DON'T modify other code.
        fn mixup<V, W>(self, other: Pointss<V, W>) -> Pointss<T, W> {
            Pointss {
                x: self.x,
                y: other.y,
            }
        }
    }
    pub fn ch10_01_06() {
        let p1 = Pointss { x: 5, y: 10 };
        let p2 = Pointss {
            x: "Hello",
            y: '中',
        };

        let p3 = p1.mixup(p2);

        assert_eq!(p3.x, 5);
        assert_eq!(p3.y, '中');

        println!("Success!");
    }
    impl Point<f32> {
        fn distance_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }
    pub fn ch10_01_07() {
        // Add decimal points and type suffixes to match the f32 implementation
        let p = Point {
            x: 5.0_f32,
            y: 10.0_f32,
        };
        println!("{}", p.distance_from_origin());
    }
}
mod const_generics {
    struct ArrayPair<T, const N: usize> {
        left: [T; N],
        right: [T; N],
    }

    // 2. Inline the trait and type paths directly into the implementation
    impl<T: std::fmt::Debug, const N: usize> std::fmt::Debug for ArrayPair<T, N> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("ArrayPair")
                .field("left", &self.left)
                .field("right", &self.right)
                .finish()
        }
    }
    pub fn ch10_02_01() {
        let pair_of_twos = ArrayPair {
            left: [1, 2],
            right: [3, 4],
        };

        println!("Small pair: {:?}", pair_of_twos);
    }
    fn foo<const N: usize>() {}

    // We introduce a second parameter `M_PLUS_1`
    fn bar<T, const M: usize, const M_PLUS_1: usize>() {
        foo::<M>();
        foo::<2021>();
        foo::<{ 20 * 100 + 20 * 10 + 1 }>();

        // Pass the pre-computed parameter directly
        foo::<M_PLUS_1>();

        let _: [u8; M];
    }

    pub fn ch10_02_02() {
        // The caller explicitly specifies or lets Rust infer both values:
        bar::<i32, 10, 11>();
    }
    struct Array<T, const N: usize> {
        data: [T; N],
    }
    pub fn ch10_02_03() {
        let arrays = [
            Array {
                data: [1.0, 2.0, 3.0],
            },
            Array {
                data: [1.0, 2.0, 3.0],
            },
            Array {
                data: [1.0, 2.0, 0.0],
            }, // Zero-padded to maintain size 3
        ];

        println!("Success!");
    }
}
mod traits {

    // Fill in the two impl blocks to make the code work.
    // DON'T modify the code in `main`.
    trait Hello {
        fn say_hi(&self) -> String {
            String::from("hi")
        }

        fn say_something(&self) -> String;
    }

    struct Student {}
    impl Hello for Student {
        fn say_something(&self) -> String {
            String::from("I'm a good student")
        }
    }
    struct Teacher {}
    impl Hello for Teacher {
        fn say_hi(&self) -> String {
            String::from("Hi, I'm your new teacher")
        }

        // Implement the required method
        fn say_something(&self) -> String {
            String::from("I'm not a bad teacher")
        }
    }
    pub fn ch10_03_01() {
        let s = Student {};
        assert_eq!(s.say_hi(), "hi");
        assert_eq!(s.say_something(), "I'm a good student");

        let t = Teacher {};
        assert_eq!(t.say_hi(), "Hi, I'm your new teacher");
        assert_eq!(t.say_something(), "I'm not a bad teacher");

        println!("Success!");
    }
    // `Centimeters`, a tuple struct that can be compared
    #[derive(PartialEq, PartialOrd)]
    struct Centimeters(f64);

    // `Inches`, a tuple struct that can be printed
    #[derive(Debug)]
    struct Inches(i32);

    impl Inches {
        fn to_centimeters(&self) -> Centimeters {
            let &Inches(inches) = self;

            Centimeters(inches as f64 * 2.54)
        }
    }

    // ADD some attributes to make the code work!
    // DON'T modify other code!
    #[derive(Debug, PartialEq, PartialOrd)]
    struct Seconds(i32);
    pub fn ch10_03_02() {
        let _one_second = Seconds(1);

        println!("One second looks like: {:?}", _one_second);
        let _this_is_true = (_one_second == _one_second);
        let _this_is_false = (_one_second > _one_second);

        let foot = Inches(12);

        println!("One foot equals {:?}", foot);

        let meter = Centimeters(100.0);

        let cmp = if foot.to_centimeters() < meter {
            "smaller"
        } else {
            "bigger"
        };

        println!("One foot is {} than one meter.", cmp);
    }
    pub fn ch10_03_03() {}
    pub fn ch10_03_04() {}
    pub fn ch10_03_05() {}
    pub fn ch10_03_06() {}
    pub fn ch10_03_07() {}
    pub fn ch10_03_08() {}
    pub fn ch10_03_09() {}
}
mod trait_object {
    pub fn ch10_04_01() {}
    pub fn ch10_04_02() {}
    pub fn ch10_04_03() {}
    pub fn ch10_04_04() {}
    pub fn ch10_04_05() {}
}
mod advanced_traits {
    pub fn ch10_05_01() {}
    pub fn ch10_05_02() {}
    pub fn ch10_05_03() {}
    pub fn ch10_05_04() {}
    pub fn ch10_05_05() {}
}
pub fn calling_main() {
    println!("\n=== Generics and Traits EXAMPLES ===\n");
    println!("=== Generics Examples ===");
    generics::ch10_01_01();
    generics::ch10_01_02();
    generics::ch10_01_03();
    generics::ch10_01_04();
    generics::ch10_01_05();
    generics::ch10_01_06();
    generics::ch10_01_07();
    println!("=== Const Generics Examples ===");
    const_generics::ch10_02_01();
    const_generics::ch10_02_02();
    const_generics::ch10_02_03();
    println!("=== Traits Examples ===");
    traits::ch10_03_01();
    traits::ch10_03_02();
    traits::ch10_03_03();
    traits::ch10_03_04();
    traits::ch10_03_05();
    traits::ch10_03_06();
    traits::ch10_03_07();
    traits::ch10_03_08();
    traits::ch10_03_09();
    println!("=== Trait Object Examples ===");
    trait_object::ch10_04_01();
    trait_object::ch10_04_02();
    trait_object::ch10_04_03();
    trait_object::ch10_04_04();
    trait_object::ch10_04_05();
    println!("=== Advanced Traits Examples ===");
    advanced_traits::ch10_05_01();
    advanced_traits::ch10_05_02();
    advanced_traits::ch10_05_03();
    advanced_traits::ch10_05_04();
    advanced_traits::ch10_05_05();
}
