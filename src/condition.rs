
pub fn run() {
    let age=10;
    // if - else
    if age>=21{
        println!("You're at least 21 years old");
    }else if age>=18 && age<21 {
        println!("You're between 18 and 21");
    }else{
        println!("You're younger than 18");
    }

    //shorthand if-else
    let age_check=if age>=21 {true} else {false};
    println!("age_check: {}",age_check);
}