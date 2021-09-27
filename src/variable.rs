// Variables hold primitive data or references to data
// Variables are immutable by default -> unless add "mut" before variable
// Rust is a block-scoped language

pub fn run(){
    let name="Brad";
    let age=32;
    //age=33; //cannot assign twice to immutable variable!!
    println!("age: {}",age);

    // add "mut" before variable to make it mutable
    let mut age2=32;
    println!("Name: {}, age: {}",name,age2);
    age2=33;
    println!("Name: {}, age: {}",name,age2);

    // define constant must also define type at the same time
    const ID:i32=001;
    println!("ID: {}",ID);

    // assign multiple variables
    let (name1,age1)=("Amy",21);
    println!("name1: {}, age1: {}",name1,age1);
}