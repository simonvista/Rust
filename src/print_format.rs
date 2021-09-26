pub fn run(){
    // print to console
    println!("hello from printFormat.rs file");
    // basic format
    println!("num: {}", 1);     //num: 1
    println!("{} is from {}","bread","wheat");  //bread is from wheat

    // positional arguments
    println!("{0} is from {1} and {0} likes to {2}","Brad","Mass","code");   //Brad is from Mass and Brad likes to code

    // named arguments
    println!("{name} likes to play {activity}",name="Lohn",activity="violin");  //Lohn likes to play violin

    //placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}",10,10,10);    //Binary: 1010 Hex: a Octal: 12

    //placeholder for debug trait
    println!("{:?}",(12,true,"hello")); //(12, true, "hello")

    //basic math
    println!("10 + 10= {}",10+10);  //10 + 10= 20
}