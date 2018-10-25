#[derive(Debug)]
struct Point{
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct xPoint{
    x: i32,
    y: i32,
    z: i32,
}


fn main(){
    let mut p1 = Point{x: 2, y: 5};
    println!("{:?}", p1.x);
    println!("{:?}", p1.y);

    let p2: Point = Point{x:5, y: 9};
    println!("{:?}", p2);


    let xp = xPoint{x: 5, y: 6};
    println!("{:?}", xp);
}