use std::env;

pub fn run() {
    let args:Vec<String>=env::args().collect();
    println!("args: {:?}",args);    //args: ["target\\debug\\tutorial.exe"]
    // cargo run hello -> args: ["target\\debug\\tutorial.exe", "hello"]
    // cargo run hello world -> args: ["target\\debug\\tutorial.exe", "hello", "world"]

    let cmd=args[1].clone();
    let name="Bob";
    let status="100%";
    println!("cmd: {}",cmd); //cargo run hello world -> cmd: hello

    if cmd=="hello" {
        println!("hi {}, how are you?",name); //cargo run hello world -> hi Bob, how are you?
    }else if cmd=="status" {
        println!("status is {}",status); ////cargo run status -> status is 100%
    }else{
        println!("invalid command");
    }
}