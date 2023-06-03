#[derive(Debug)]
pub enum IpAddrKind {
    V4,
    V6,
}

pub fn route(ipkind: IpAddrKind) {
    println!("ipkind: {ipkind:#?}");
}

fn main() {
    let ip4 = IpAddrKind::V4;
    let ip6 = IpAddrKind::V6;
    println!("Ip4: {ip4:?}, Ip6: {ip6:?}");

    route(IpAddrKind::V4);
}
