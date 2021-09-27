// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - Use when you need to modify or own string data
pub fn run() {
    // immutable fix length string
    let hello="hello";  //&str type string can't be altered even 'mut' is applied before variable
    println!("{}",hello);
    // Growable, heap-allocated data structure
    let mut hello1=String::from("Hello ");
    println!("{}",hello1);
    // get string length
    println!("hello1 length: {}",hello1.len());

    //change string
    hello1.push('W');
    println!("{}",hello1);
    hello1.push_str("orld");
    println!("{}",hello1);  //Hello World

    //capacity in bytes
    println!("capacity: {}",hello1.capacity()); //12

    //check if empty 
    println!("is empty?: {}",hello1.is_empty());

    //contains
    println!("contain sub string \"or\"?: {}",hello1.contains("or"));

    //replace
    println!("{}",hello1.replace("World", "All"));
    
    //loop throuh string by whitespace
    for word in hello1.split(" ") {
        println!("{}",word);
    }
    for word in hello1.split_whitespace() {
        println!("{}",word);
    }

    //create string w/ capacity
    let mut str=String::with_capacity(10);
    str.push('a');
    str.push('b');    
    println!("{}",str);

    //assertion test
    assert_eq!(2,str.len()); //only show error if it fails
    assert_eq!(10,str.capacity());
}