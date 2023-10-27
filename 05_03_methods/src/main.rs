
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        (self.width > other.width && self.height > other.height) || (self.height > other.width && self.width > other.height)
    }

}

impl Rectangle {

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

}


fn main() {
    let rec = Rectangle {
        width: 30,
        height: 50,
    };

    let rec1 = Rectangle {
        width: 40,
        height: 50,
    };

    let rec2 = Rectangle {
        width: 30,
        height: 60,
    };

    let rec3 = Rectangle {
        width: 40,
        height: 20,
    };

    println!("the area is: {}", rec.area());

    if rec.width() {
        println!("The rec width is non zero");
    }

    println!("rec can hold rec1? {}", rec.can_hold(&rec1));
    println!("rec can hold rec2? {}", rec.can_hold(&rec2));
    println!("rec can hold rec3? {}", rec.can_hold(&rec3));


    println!("This is a new square: {:#?}", Rectangle::square(7));

}




