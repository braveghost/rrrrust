use restaurant;
pub mod my_mod;

fn main() {
    my_mod::my_mod();
    my_mod::tests::test1();
    restaurant::eat_at_restaurant();
    restaurant::eat();
    restaurant::front_of_house::hosting::add_to_waitlist()
}
