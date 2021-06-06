pub struct Breakfast{
    pub toast:String,
    seasonal_fruit: String,
}

impl Breakfast {
    pub fn summer(toast:&str)-> Breakfast{
        Breakfast{toast: String::from(toast),seasonal_fruit:String::from("peaches")}
    }
}

fn fix_incorrect_order(){
    cook_order();
    // super 是 back_of_house的父模块
    super::serve_order()
}

fn cook_order(){
    println!("cook order");
}

pub enum Appetizer{
    Soup,
    Salad,
}