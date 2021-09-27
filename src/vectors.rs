// Vector is resizable array
use std::mem;
pub fn run() {
    //immtable
    let nums=vec![1,2,3,4,5];
    println!("{:?}",nums);
    //get single value
    println!("1st element of nums: {}",nums[0]); //1

    let mut nums=vec![1,2,3,4,51];
    //change value
    nums[2]=-1;
    println!("{:?}",nums);

    //add element to vector
    nums.push(-1);
    println!("nums: {:?}",nums);
    //pop last element
    nums.pop();

    //get array length
    println!("nums length: {}",nums.len());

    //array is stack allocated
    // println!("nums takes {} bytes",std::mem::size_of_val(&nums));
    println!("nums takes {} bytes",mem::size_of_val(&nums));

    //get slice
    let slice=&nums;
    println!("slice: {:?}",slice);  //slice: [1, 2, -1, 4, 51]
    let slice=&nums[1..3];   
    println!("slice: {:?}",slice);  //slice: [2, -1]
    
    //loop through vector
    for num in nums.iter() {
        print!("{}, ",num);
    }

    //loop & mutate vector
    for elem in nums.iter_mut() {
        *elem *=2;
    }
    println!("\n{:?}",nums);    //[2, 4, -2, 8, 102]
}