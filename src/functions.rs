
pub fn run() {
    greeting("hi", "John");
    let sum=add(5,-7);
    println!("5-7={}",sum);
    //closure
    let n3=5;
    let sum_=|n1:i32,n2:i32| n1+n2+n3;
    println!("{}",sum_(5,-10));
}

fn greeting(greet:&str,name:&str){
    println!("{} {}, nice to meet you",greet,name);
}
fn add(n1:i32,n2:i32)->i32{
    // return n1+n2;
    n1+n2
}