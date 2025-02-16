mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}
pub use crate::front_of_house::hosting;

mod customer {
    use crate::hosting;
    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
}

fn main() {
    customer::eat_at_restaurant();
}
