
pub fn creating_and_accessing() {
    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];

    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element :("),
    }

    let does_not_exist = v.get(100); // this passes as it returns None!
    //let does_not_exist = &v[100]; // panics

    let first = &v[0];

    // v.push(6); - error! this will not work as 'let first = &v[0];' does immutable borrow

    println!("The first element is: {first}");
}

pub fn loops() {
    let mut v  = vec![1, 2, 3, 4, 5, 6];

    for i in &v {
        println!{"{i}"};
    }

    for i in &mut v {
        *i += 50;
    }

    for i in &v {
        println!{"{i}"};
    }
}

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

pub fn enums() {
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for i in &row {
        println!("{i:?}");
    }
}