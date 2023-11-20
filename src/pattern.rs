struct Point{
 x:i32,
 y:i32
}

enum Message {
 Move{x:i32,y:i32},
 Write(String),
    
}

pub fn run(){
 let p:Point=Point { x: 10, y: 20 };
 let msg=Message::Write("Ronnie".to_string());
 println!("Running..");
 let x:i32=9;
 match x{
  1|2=>println!("One and Two"),
  2|3=> println!("Two and Three"),
  4..=10 => println!("It's Four and five"),
  _=> println!("Anything"),

 }
 match p {
  Point{x:10,y:20}=>println!("Matched"),
  _=>println!("Did Not Matched"),
     
 }

 match msg {
  Message::Move { x, y }=>println!("Move"),
  Message::Write(text)=>println!("Message is {}",text),
     
 }

 let o=Some(7);
 if let Some(_) = o {
  println!("Found the value: {:?}",o)
     
 }
}