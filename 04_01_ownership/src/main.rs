

fn main() {
    let x: u32 = 3;

    let y = copies_variable(x);

    println!("{}", x); // this is valid because x was deep copied

    {
        let z = 3;
    } // z out of scope

    let s = String::new();

    let s2 = moves_variable(s);

    // println!("{}", s); doesn't work! it's becase s was borrowed by moves_variable and then moved into s2
    println!("{}", s2); // This works though :)
}

fn copies_variable(x: u32) -> u32 {
    x
}

fn moves_variable(s: String) -> String {
    s
}