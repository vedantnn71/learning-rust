// #[derive(Debug)]
// enum IpAddrKind {
//     V4(u8, u8, u8, u8),
//     V6(String),
// }
//
// fn main() {
//     // let home = IpAddrKind::V4(127, 0, 0, 1);
//     // let loopback = IpAddrKind::V6(String::from("::1"));
//     //
//     // println!("{:#?}", home);
//
//     let none: Option<i8> = None;
//     let some: Option<i8> = Some(8);
//
//     println!("{:#?}", none+some);
// }


#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    unimplemented!();
}
