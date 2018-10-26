#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main(){
    let number: &mut i32 = &mut 4;
    *number = 10;

    let mut point = Point{x: 3, y: 7};
    inspect(&point);
    move_point(&mut point, 3, 3);
    inspect(&point);


    let mut buffer = String::from("Hello");
    //let slice = &buffer[1..];
    //slice.push_str(" World"); 
    transform_string(&mut buffer);
    String::push_str(&mut buffer, " World");
    println!("{}", buffer);
}

fn transform_string(s: &mut String){
    *s = *s[1..];
}

fn move_point(p: &mut Point, x: i32, y: i32) {
    p.x += x;
    p.y += y;
} 

fn inspect(p: &Point){
    println!("{:?}", p);
}
