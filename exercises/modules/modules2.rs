// modules2.rs
// Make me compile! Execute `rustlings hint modules2` for hints :)

mod delicious_snacks {
    pub use self::fruits::PEAR as fruit;
    pub use self::veggies::CUCUMBER as veggie;

    pub mod fruits {
        pub const PEAR: &'static str = "Pear";
        pub const APPLE: &'static str = "Apple";
    }

    mod veggies {
        pub const CUCUMBER: &'static str = "Cucumber";
        pub const CARROT: &'static str = "Carrot";
    }
}

use delicious_snacks::fruits;

fn main() {
    println!(
        "favorite snacks: {} and {}",
        fruits::PEAR,
        delicious_snacks::veggie,
    );
}
