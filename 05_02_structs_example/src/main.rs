

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let width = 30;
    let height = 50;

    let dimensions = (30, 50);

    let rec = Rectangle {
        width: dbg!(2 * 30),
        height: 50,
    };

    println!("The area of the rectangle is {} square pixels.", area(width, height));
    println!("The area of the rectangle is {} square pixels.", area_tuple(dimensions));
    println!("The area of the rectangle is {} square pixels.", area_struct(&rec));

    println!("My struct is so nice! {:?}", rec);
    println!("This struct is even prettier! {:#?}", rec);

    dbg!(rec);

}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

