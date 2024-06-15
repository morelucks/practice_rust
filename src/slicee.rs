
pub fn run(){
    let num:[i32; 4]=[2, 3,4, 5];
    let slice:&[i32]=&num;
    let sl:&[i32]=&num[0..3];
    println!("slice {:?}", slice);
    println!("{:?}", num);
    println!("{:?}", sl);

}