fn plus_o(initial_result: & mut i32) { *initial_result += 1 }
fn plus(first: i32, second: i32) -> f32 { (first + second) as f32 }
fn sub(first: i32, second: f32) -> f32 { second - first as f32  }
fn mult(first: i32, second: f32) -> f32 { first as f32 * second }
fn div(first: i32, second: f32) -> f32 { second / first as f32 }

fn main(){
    let first = 2;
    let second = 1;
    let mut current_result;
    let mut initial_result = 0;
    plus_o(&mut initial_result);
    println!("Initial result:{}", initial_result);
    
    current_result = plus(first, second);
    println!("{}", current_result);
    current_result = mult(first, current_result);
    println!("{}", current_result);
    current_result = sub(first, current_result);
    println!("{}", current_result);
    current_result = div(first, current_result);
 
    println!("{}", current_result);
}