// 定义模块指向 front_of_house.rs文件
// pub 允许暴露到外部引用 即公共
pub mod front_of_house;
// 定义模块指向 back_of_house.rs文件
mod back_of_house;


mod pay {
    fn pay(amount: i64) {
        println!("pay amount: {}", amount);
    }
}

use crate::front_of_house::hosting::add_to_waitlist as atw;
use crate::front_of_house::hosting; // 绝对路径引用
// use self::front_of_house::hosting; // 相对路径引用

pub fn eat_at_restaurant() {
    // 绝对路径执行
    crate::front_of_house::hosting::add_to_waitlist();
    // 相对路径
    front_of_house::hosting::add_to_waitlist();
}

fn serve_order() {
    println!("serve order");
}

pub fn eat() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("wheat");
    println!("I'd like {} toast please", meal.toast);
    // meal.seasonal_fruit = String::from("blueberries"); // 内部未暴露 编译失败

    let _ = back_of_house::Appetizer::Salad;

    atw();
    hosting::add_to_waitlist();
}

