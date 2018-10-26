use std::collections::HashMap;

fn message(arg: Option<&str>) {
    match arg {
        Some(arg @ "ValueB") => println!("El value es : {}", arg),
        Some(arg)  => println!("{}", arg),
        None  => println!("nothing"),
    }
}

fn main(){
    // let mut m: HashMap<String, String> = HashMap.new();
    let mut m: HashMap<&str, &str> = HashMap::new();

    m.insert("a", "valueA");
    m.insert("b", "valueB");
    m.insert("c", "valueC");

    message(m.get("b").map(|v| *v));
    message(m.get("a").map(|v| *v));
    message(m.get("x").map(|v| *v));
}