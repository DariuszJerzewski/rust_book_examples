
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
struct ComplexType {
    name: String,
    description: String,
    version: u8,
}

#[derive(Debug)]
struct SimpleType {
    name: String,
}

#[derive(Debug)]
enum TypeKind {
    Simple(SimpleType),
    Complex(ComplexType),
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    println!("Is v4? {}", is_v4(four));
    println!("Is v4? {}", is_v4(six));

    let localhost = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    println!("This is my address: {:?}", localhost);
    println!("This is my address: {:?}", loopback);

    let complex = TypeKind::Complex(ComplexType {
        name: String::from("test"),
        description: String::from("description"),
        version: 1,
    });

    let simple = TypeKind::Simple(SimpleType {
        name: String::from("test2"),
    });

    println!("complex: {:#?}", complex);
    println!("simple: {:#?}", simple);
}

fn is_v4(ip_kind: IpAddrKind) -> bool {
    match ip_kind {
        IpAddrKind::V4 => true,
        _ => false,
    }
}
