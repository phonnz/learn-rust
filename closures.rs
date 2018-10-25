fn main(){
    // let range = 1..5 ;
    let range = (1,1,2,3,4);
    let value = range.map(|v| -> i32 { v + 1 });

    
    println!("{:?}", value )
}