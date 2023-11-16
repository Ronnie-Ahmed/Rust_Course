enum Movement{
 Up,
 Down,Left,Right
}

fn move_avater(m:Movement){
 match m{
  Movement::Down=>println!("Moving Down"),
  Movement::Left=>println!("Moving Left"),
  Movement::Right=>println!("Moving Right"),
  Movement::Up=>println!("Moving Up")
 };

}

pub fn run(){
 println!("Running...");
 move_avater(Movement::Down);
 move_avater(Movement::Left);
 move_avater(Movement::Right);
 move_avater(Movement::Up);
}