use std::ops::Add;

#[derive(Debug)]
struct GenericStruct<X>{
    value: X,
}

fn add<T:Add<Output=T>> (first: T, second: T) -> T {
    first + second
}


fn main(){
    let x: GenericStruct<String> = GenericStruct{ value: "done".to_string(), };
    println!("{:?}", x);
    let y: GenericStruct<i32> = GenericStruct{ value: 34, };
    println!("{:?}", y);

    println!("{:?}", add(1.5, 5.0))
}