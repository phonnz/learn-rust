#[derive(Debug, Clone, Copy)]
struct Dot {
    x: i32,
    y: i32,
}

fn eat(dot: Dot) {
    
    println!("Dot => {:?}", dot);
}


fn eat(dot: &mut Dot) {
    dot.x = dot.x + 5;
    println!("{:?}", dot);
}


fn  main(){
    let mut dot = Dot{x:5, y: 15};

    eat(&mut dot.clone());
    eat(dot);
    println("{:?}", dot);
}