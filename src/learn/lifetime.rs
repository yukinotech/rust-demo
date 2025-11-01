struct Person<'a> {
    name: &'a str,
}

#[allow(dead_code)]
pub fn init() {
    let name = String::from("Alice");
    let p = Person { name: &name };
    println!("{}", p.name);
}
