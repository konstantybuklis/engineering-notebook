#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[allow(dead_code)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // some logic
    }
}

#[allow(dead_code)]
fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);
    route(six);

    //     #[derive(Debug)]
    //     struct IpAddr {
    //         kind: IpAddrKind,
    //         address: String,
    //     }
    //
    //     let home = IpAddr {
    //         kind: IpAddrKind::V4,
    //         address: String::from("127.0.0.1"),
    //     };
    //
    //     println!("{home:?}");
    //
    //     let loopback = IpAddr {
    //         kind: IpAddrKind::V6,
    //         address: String::from("::1"),
    //     };
    //
    //     println!("{loopback:?}");

    #[derive(Debug)]
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    println!("home = {home:?}, loopback = {loopback:?}");

    let m = Message::Write(String::from("Hello, World!"));
    m.call();

    let some_number = Some(5);
    let some_char = Some('🦀');
    let absent_number: Option<i32> = None;

    println!("{some_number:?}, {some_char:?}, {absent_number:?}");

    // Some(5) + None == 5 + 10
    let result = some_number.unwrap_or(42) + absent_number.unwrap_or(10);
    println!("The value of result is: {result}");
}

fn route(_ip_kind: IpAddrKind) {}
