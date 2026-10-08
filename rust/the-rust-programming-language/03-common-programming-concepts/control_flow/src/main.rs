fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if number != 0 {
        println!("number was something other than zero")
    }

    let condition = true;

    let result = if condition { 5 } else { 6 };
    println!("The value of result is: {result}");

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The value of result is: {result}");

    let mut counter = 0;

    'counting_loop: loop {
        println!("counter = {counter}");

        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");

            if remaining == 9 {
                break;
            }

            if counter == 2 {
                break 'counting_loop;
            }

            remaining -= 1;
        }

        counter += 1;
    }

    println!("End counter = {counter}");

    let mut number = 3;

    while number != 0 {
        println!("number = {number}");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("{element}");
    }

    for number in (1..=5).rev() {
        println!("{number}");
    }

    println!("LIFTOFF!!!");
}
