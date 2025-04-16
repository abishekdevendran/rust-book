mod back_of_house;

mod saas {
    mod backend {
        fn handle_request() {}
    }
    pub (crate) mod frontend {
        pub fn handle_request() {}
    }
}

pub fn handle_request() {
    saas::frontend::handle_request();
    crate::saas::frontend::handle_request();
}

pub fn eat_at_restaurant() {
    let mut meal = back_of_house::hosting::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
}