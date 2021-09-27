// Arrays - Fixed list where elements are the same data types
use std::mem;
pub fn run() {
    //immtable
    let nums=[1,2,3,4,5];
    println!("{:?}",nums);
    //get single value
    println!("1st element of nums: {}",nums[0]); //1

    let mut nums=[1,2,3,4,51];
    //change value
    nums[2]=-1;
    println!("{:?}",nums);

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
}