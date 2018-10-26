use std::collections::HashMap;

fn main(){
    // let mut m: HashMap<String, String> = HashMap.new();
    let mut m = HashMap::new();

    m.insert("a", "valueA");
    m.insert("b", "valueB");
    m.insert("c", "valueC");

    // let r = m.get("b");

    match m.get("b") {
        Some(v) => println!("{}", v),
        None  => println!("nothing")
    }

    match m.get("x") {
        Some(v) => println!("{}", v),
        None  => println!("nothing")
    }
}