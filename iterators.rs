fn main(){
    let v = vec![1,2,3,4,5];
    let mut i  = v.iter(); // Vec::iter(&mut v)
    println!("{:?}", i.next());
}