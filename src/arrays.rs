pub fn run() {
    let mut nums: [i64; 4] = [2, 4, 2, 4];
    // re assign
    nums[2] = 3;
    let y = 7;
    let name = "lucky luck";
     
    // checking size of variable on memory 
    println!("array occupied {} byte", std::mem::size_of_val(&nums));
    println!("var occupied {} byte", std::mem::size_of_val(&y));
    println!("str occupied {} byte", std::mem::size_of_val(&name));

    // println!("{:?}", nums);
}
