#[derive(Debug)]
enum Movement {
    Right(i32),
    Left(i32),
    Up(i32),
    Down(i32),
}

fn main(){
    let movement = Movement::Left(5);
    println!("{:?}", movement)
    //println!("{}", unsafe { std::intrinsics::type_name::<T>() })
}