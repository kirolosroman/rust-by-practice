#![allow(unused)]
mod methods_assoc_funcs {
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        // Complete the area method which return the area of a Rectangle.
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }

    pub fn ch_09_01() {
        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };

        assert_eq!(rect1.area(), 1500);

        println!("Success!");
    }
    #[derive(Debug)]
    struct TrafficLight {
        color: String,
    }

    impl TrafficLight {
        pub fn show_state(self: &Self) {
            println!("the current state is {}", self.color);
        }

        // Fill in the blank, DON'T use any variants of `Self`.
        pub fn change_state(self: &mut TrafficLight) {
            self.color = "green".to_string()
        }
        pub fn new() -> Self {
            Self {
                color: "red".to_string(),
            }
        }

        pub fn get_state(&self) -> &str {
            &self.color
        }
    }
    pub fn ch_09_02() {
        let light = TrafficLight {
            color: "red".to_owned(),
        };
        // Don't take the ownership of `light` here.
        light.show_state();
        // ... Otherwise, there will be an error below
        println!("{:?}", light);
    }

    pub fn ch_09_03() {
        println!("Success!");
    }
    pub fn ch_09_04() {
        let light = TrafficLight::new();
        assert_eq!(light.get_state(), "red");

        println!("Success!");
    }
    impl Rectangle {
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }

    pub fn ch_09_05() {
        println!("Success!");
    }
    #[derive(Debug)]
    enum TrafficLightColor {
        Red,
        Yellow,
        Green,
    }
    // Implement TrafficLightColor with a method.
    impl TrafficLightColor {
        pub fn color(&self) -> &str {
            match self {
                TrafficLightColor::Red => "red",
                TrafficLightColor::Yellow => "yellow",
                TrafficLightColor::Green => "green",
            }
        }
    }
    pub fn ch_09_06() {
        let c = TrafficLightColor::Yellow;

        assert_eq!(c.color(), "yellow");

        println!("{:?}", c);
    }
}
pub fn calling_main() {
    println!("\n=== Methods and Associated Functions EXAMPLES ===\n");
    methods_assoc_funcs::ch_09_01();
    methods_assoc_funcs::ch_09_02();
    methods_assoc_funcs::ch_09_03();
    methods_assoc_funcs::ch_09_04();
    methods_assoc_funcs::ch_09_05();
    methods_assoc_funcs::ch_09_06();
}
