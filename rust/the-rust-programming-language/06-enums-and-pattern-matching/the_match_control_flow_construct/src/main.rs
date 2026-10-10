#[derive(Debug)]
#[allow(dead_code)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

#[derive(Debug)]
#[allow(dead_code)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

impl Coin {
    fn value_in_cents(&self) -> u8 {
        match self {
            Self::Penny => {
                println!("Lucky penny!");
                1
            }
            Self::Nickel => 5,
            Self::Dime => 10,
            Self::Quarter(state) => {
                println!("State quarter from {state:?}!");
                25
            }
        }
    }
}

fn main() {
    let p = Coin::Penny;
    println!("p = {}", p.value_in_cents());

    let q = Coin::Quarter(UsState::Alabama);
    println!("q = {}", q.value_in_cents());

    let some_number = Some(5);
    let result = plus_one(some_number);
    println!("The value of result is: {result:?}");

    let dice_roll = 9;

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // other => move_player(other),
        _ => reroll(),
    }
}

fn plus_one(some_number: Option<i32>) -> Option<i32> {
    match some_number {
        Some(number) => Some(number + 1),
        None => None,
    }
}

fn add_fancy_hat() {}

fn remove_fancy_hat() {}

#[allow(dead_code)]
fn move_player(_num_spaces: u8) {}

fn reroll() {}
