#[derive(Debug)]
struct Cmd<'a> {
    binary: &'a str,
}

fn main(){
    let s =String::from("hi");
    let cmd =Cmd{ binary: s.as_str()};

    println!("{:?}", cmd);
}