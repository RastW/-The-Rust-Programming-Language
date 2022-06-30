enum IpAddKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

struct  IpAddr {
    kind: IpAddKind,
    address: String,
}

fn main() {
    let four = IpAddKind::V4(0, 0, 0, 0);
    let six = IpAddKind::V6(String::from("::1"));

    // route(four);
    // route(six);

    // let home = IpAddr {
    //     kind: IpAddKind::V4,
    //     address: String::from("127.0.0.1"),
    // };
    // let home = IpAddr {
    //     kind: IpAddKind::V6,
    //     address: String::from("::1"),
    // };

    // Option --------------------------------
    let s = Some(5);

    let some_string = Some(String::from("A String"));

    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // let sum = x + y;
}

fn route(ip_kind: IpAddKind){}
