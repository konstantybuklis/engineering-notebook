#[allow(dead_code)]
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer(back_of_house::Toast::Rye);
    meal.toast = back_of_house::Toast::Wheat;
    println!("I'd like {:?} toast please", meal.toast);

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
    println!("order1 = {order1:?}, order2 = {order2:?}");
}

fn deliver_order() {}

#[allow(dead_code)]
mod back_of_house {
    #[derive(Debug)]
    pub enum Toast {
        Rye,
        Wheat,
    }

    pub struct Breakfast {
        pub toast: Toast,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: Toast) -> Self {
            Self {
                toast: toast,
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    #[derive(Debug)]
    pub enum Appetizer {
        Soup,
        Salad,
    }

    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}

// module tree

// crate
//  └── front_of_house
//      ├── hosting
//      │   ├── add_to_waitlist
//      │   └── seat_at_table
//      └── serving
//          ├── take_order
//          ├── serve_order
//          └── take_payment
