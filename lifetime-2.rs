fn return_string() -> &'static str {
    "Hi!"
}

fn return_other_string() -> String {
    String::from("Hi again!")
}

fn return_referenced_string(str: &String) -> &String {
    str
}

fn return_lifetime_string<'a>(str: &'a String) -> &'a String {
    str
}

// not valid
// fn return_lifetime_string<'a, 'b>(str: &'a String) -> &'b String {
//     str
// }

fn return_lifetime_string<'a, 'b: 'a>(str: &'a String) -> &'b String {
    str
}

fn main(){
    let str = return_string();
    println!("{}", str);
    let str2 = return_other_string();
    println!("{}", str2);

}