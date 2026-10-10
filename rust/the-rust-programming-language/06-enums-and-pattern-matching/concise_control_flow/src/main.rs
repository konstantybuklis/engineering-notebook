#[derive(Debug)]
#[allow(dead_code)]
enum UsState {
    Alabama,
    Alaska,
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
        }
    }
}

#[allow(dead_code)]
enum Coin {
    Penny,
    Quarter(UsState),
}

impl Coin {
    fn describe_state_quarter(&self) -> Option<String> {
        let Coin::Quarter(state) = self else {
            return None;
        };

        if state.existed_in(1900) {
            Some(format!("{state:?} is pretty old, for America!"))
        } else {
            Some(format!("{state:?} is relatively new."))
        }
    }
}

fn main() {
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }

    let coin = get_coin();
    let mut count = 0;

    if let Coin::Quarter(state) = coin {
        println!("State quarter from {state:?}!");
    } else {
        count += 1;
    }

    println!("{count}");

    let coin = get_coin();
    coin.describe_state_quarter();
}

fn get_coin() -> Coin {
    Coin::Quarter(UsState::Alabama)
}
