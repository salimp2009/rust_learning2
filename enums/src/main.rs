#![allow(dead_code)]
#[derive(Debug)]
pub enum IpAddrKind {
    V4,
    V6,
}

pub fn route(ipkind: IpAddrKind) {
    println!("ipkind: {ipkind:#?}");
}

#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

// #[allow(dead_code)]
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        if let Message::Write(msg) = self {
            println!("{msg:#?}");
        } else {
            println!("{self:#?}");
        }
    }
}

fn main() {
    let ip4 = IpAddrKind::V4;
    let ip6 = IpAddrKind::V6;
    println!("Ip4: {ip4:?}, Ip6: {ip6:?}");

    let home_server = IpAddr::V4("127.0.0.1".to_string());
    let loop_back = IpAddr::V6("::1".to_string());
    println!("home address: {home_server:#?}");
    println!("home address: {loop_back:#?}");

    let msg1 = Message::Write(String::from("hi there"));
    msg1.call();
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    println!("{}", x + y.unwrap_or(0));

    route(IpAddrKind::V4);
}
