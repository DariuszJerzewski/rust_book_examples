




fn main() {

    let mut s = String::from("Hello!");

    let size = calculate_len(&s); // references "&", can be dereferenced by using "*"

    println!("{}", size); // works :)

    change_var_working(&mut s);

    let r1 = &s;
    let r2 = &s;

    // let r3 = &mut s; // this wouldn't work here, as it's a mutable borrow and previous borrows weren't returned!

    println!("{}, {}", r1, r2);

    let r3 = &mut s;  // This works because previous borrows were already returned, as they were moved out of scope

    println!("{}", r3);

    //let reference_to_nothing = dangle();
    let my_string = no_dangle();

    
}

fn calculate_len(s: &String) -> usize {

    s.len()

}

// fn change_var_not_working(s: &String) {  // This will not work, because the borrowed value is not a mutable reference
//     s.push_str("test");
// }

fn change_var_working(s: &mut String) {

    s.push_str("test");

}

// fn dangle() -> &String { // This doesn't work as we'd be returning a invalid reference as "s" would be out of scope
//     let s = String::new();
//
//     &s
// }


fn no_dangle() -> String { // This works because the value is moved, rather than borrowed!
    let s = String::new();

    s
}