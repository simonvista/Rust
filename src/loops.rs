
pub fn run() {
    let mut cnt=0;
    //infinite loop
    loop {
       cnt+=1;
       print!("{}, ",cnt);
       if cnt==10{
          break; 
       }
    }
    println!("\n------------");
    cnt=1;
    //while loop (FizzBuzz)
    while cnt<=20 {
        if cnt%15==0{
            println!("FizzBuzz");
        }
        else if cnt%5==0 {
            println!("Buzz");
        }
        else if cnt%3==0 {
            println!("Fizz");
        }else {
            println!("{}",cnt);
        }
        cnt+=1;
    }
    println!("--------------");
    //for range loop
    for cnt in 1..20 {
        if cnt%15==0{
            println!("FizzBuzz");
        }
        else if cnt%5==0 {
            println!("Buzz");
        }
        else if cnt%3==0 {
            println!("Fizz");
        }else {
            println!("{}",cnt);
        }
    }
}