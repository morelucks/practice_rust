pub fn run(){
    greetings("hell", "lucky");
    //bind fn
    let sums=add(4, 2);
    println!("results {}", sums);
}

fn greetings(greet:&str, name:&str){
    println!("{} {}", greet, name)
     
}
fn add(num1:i32, num2:i32)->i32{
 num1 + num2
}