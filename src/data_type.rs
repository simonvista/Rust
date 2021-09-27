/*
Primitive Types--
Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (number of bits they take in memory)
Floats: f32, f64
Boolean (bool)
Characters (char)
Tuples
Arrays
*/
/* 
Rust is a statically typed language, which means that it must know the types of all variables at compile time, 
however, the compiler can usually infer what type we want to use based on the value and how we use it.
*/
pub fn run() {
    // let key word -> Bind a value to a variable.
    let x=1;
    let y=2.5;
    let z:i64=1235677733;
    // find max of data types
    println!("max i32: {}",std::i32::MAX);
    println!("max i64: {}",std::i64::MAX);

    // boolean
    let is_active=true;
    // placeholder for debug trait
    println!("{:?}",(x,y,z,is_active)); //(1, 2.5, 1235677733, true)
    // get boolean from expression
    let is_bigger=9>2;
    println!("is_bigger={}",is_bigger);
    let a1='a';
    let face='\u{1F600}';
    println!("{:?}",(a1,face));

    
}