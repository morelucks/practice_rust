pub fn run(){
    let  mut count=0;
    // loop {
    //     println!("{}", count);
    //     if count==10 {
    //         break;
    //     }
    //     count +=1;
    // }
    while count<=50 {
        if count%15==0 {
            println!("fizzbuzz")
        }else if count%5==0 {
            println!("buzz")
        }else {
            println!("{}", count)
        }
        count+=1;
    }
}