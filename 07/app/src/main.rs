use my_lib::front_of_house::hosting;
use my_lib::front_of_house::serving;

fn main() {
    println!("Welcome to the restaurant!");
    hosting::add_to_waitlist();
    hosting::seat_at_table();
    serving::take_order();
    serving::serve_order();
    serving::take_payment();
    println!("Thank you for dining with us!");
}
