
enum Movement{
    Up,
    Left,
    Right,
    Down
}
fn move_av(m:Movement){
    match m {
        Movement::Down=>println!("moving"),
        Movement::Left=>println!("moving left"),
        Movement::Right=>println!("moving right"), 
        Movement::Up=>println!("moving up"),
    }
}

pub fn run(){
 let av1=Movement::Down;
  let av2=Movement::Left;
 let av3=Movement::Right;
 let av4=Movement::Up;

 move_av(av1);
  move_av(av2);
 move_av(av3);
 move_av(av4);

}