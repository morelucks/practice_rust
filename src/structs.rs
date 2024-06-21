// there two types of struct. traditional struct and tupple struct

// traditional struct
// struct Color{
//     red:u8,
//     blue:u8,
//     green:u8,
// }
struct Color(u8, u8, u8);

pub fn run(){
    // let mut c=Color{
    //     red:200,
    //     blue:0,
    //     green:0,
    // };
    // c.red=100;
    
    // println!("{} {} {}", c.red, c.blue, c.green)
    
    //     red:200,
    //     blue:0,
    //     green:0,
    // };
    // c.red=100;
    
    // println!("{} {} {}", c.red, c.blue, c.green)
        let c=Color(200,0,0);
    println!("{} {} {}", c.0, c.1, c.2)


}