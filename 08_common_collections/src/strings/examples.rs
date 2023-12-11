
pub fn creating_and_modifying() {
    let data = "initial contents";

    let mut s = String::new();
    let s = data.to_string();
    let s = "initial contents".to_string();
    let s = String::from("initial contents");

    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    let mut s = String::new();
    s.push_str("foo");
    let s2 = "bar";
    s.push_str(s2); // push_str doesn't take ownership
    println!("s2 is {s2}");

    s.push('t');
    println!("s is now {s}");

    let s3 = String::from("example");
    let s4 = s + &s3; // addition of two strings takes ownership of first string, the second must be a str slice!

    let ss1 = String::from("tic");
    let ss2 = String::from("tac");
    let ss3 = String::from("toe");

    let ss = format!("{ss1}-{ss2}-{ss3}");

    let hello = "Здравствуйте";

    let s = &hello[0..4]; // 3A

    for c in "Зд".chars() {
        println!("{c}");
    }

    for b in "Зд".bytes() {
        println!("{b}");
    }
}

