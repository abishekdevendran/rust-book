enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

impl IpAddrKind {
    fn new (v: IpAddrKind) -> IpAddrKind {
        v
    }
}

fn main() {
    println!("Hello, world!");
    let home = IpAddrKind::new(IpAddrKind::V4(127, 0, 0, 1));
}
