use std::net::{Ipv4Addr, Ipv6Addr};

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

fn route(ip_kind: IpAddrKind) {
    println!("{:?}", ip_kind);
}

//Storing the data and IpAddrKind variant of an IP address using a struct
#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

//definition of the IpAddrString enum says that both V4 and V6 variants will have associated String values
#[derive(Debug)]
enum IpAddrString {
    V4(String),
    V6(String),
}

//to store V4 addresses as four u8 values but still express V6 addresses as one String
#[derive(Debug)]
enum IpAddrU8 {
    V4(u8, u8, u8, u8),
    V6(String),
}

//file:///home/caddy/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/share/doc/rust/html/std/net/enum.IpAddr.html
#[derive(Debug)]
enum IpAddr_lib {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

//A Message enum whose variants each store different amounts and types of values
#[derive(Debug)]
enum Message {
    Quit,                       //Quit has no data associated with it at all.
    Move { x: i32, y: i32 },    //Move has named fields, like a struct does.
    Write(String),              //Write includes a single String.
    ChangeColor(i32, i32, i32), //ChangeColor includes three i32 values.
}
impl Message {
    fn call(&self) {
        dbg!(self);
    }
}
struct QuitMessage; //unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); //tuple struct
struct ChangeColorMessage(i32, i32, i32); //tuple struct

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    dbg!(home);

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    println!("{:?}", loopback);

    let home = IpAddrString::V4(String::from("127.0.0.1"));
    println!("{:?}", home);

    let loopback = IpAddrString::V6(String::from("::1"));
    println!("{:?}", loopback);

    let home = IpAddrU8::V4(127, 0, 0, 1);
    println!("{:?}", home);

    let loopback = IpAddrU8::V6(String::from("::1"));
    println!("{:?}", loopback);

    let localhost_v4 = IpAddr_lib::V4(Ipv4Addr::new(127, 0, 0, 1));
    let localhost_v6 = IpAddr_lib::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1));
    dbg!(localhost_v4);
    dbg!(localhost_v6);

    let m = Message::Write(String::from("hello"));
    m.call();

    //file:///home/caddy/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/share/doc/rust/html/std/option/enum.Option.html
    let some_number = Some(5);
    let some_char = Some('e');
    //The Option Enum and Its Advantages Over Null Values
    //The Option<T> enum is so useful that it’s even included in the prelude;
    //you don’t need to bring it into scope explicitly.
    //Its variants are also included in the prelude: you can use Some and None directly without the Option:: prefix.
    //The Option<T> enum is still just a regular enum, and Some(T) and None are still variants of type Option<T>.
    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    //let sum=x+y;
    //have to convert an Option<T> to a T before you can perform T operations with it
    let sum = x + y.unwrap();
    dbg!(sum);
}
