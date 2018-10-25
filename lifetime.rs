fn with_lifetime<'a>(thing: &'a str) -> &'a str {
    thing
}


fn main(){
    let foo = "foo";
    // with_lifetime(foo)
    // with_lifetime(with_lifetime(foo))
    println!("{}", with_lifetime(foo))
}