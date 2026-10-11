// idiomatic way for struct and enum
use std::{collections::HashMap, fmt::Result, io::Result as IoResult};

use rand::prelude::*;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);

    // use re-exported function
    restaurant::hosting::add_to_waitlist();

    let secret_number = rand::rng().random_range(1..=100);
    println!("{secret_number}");
}

#[allow(dead_code)]
fn function1() -> Result {
    Ok(())
}

#[allow(dead_code)]
fn function2() -> IoResult<()> {
    Ok(())
}
