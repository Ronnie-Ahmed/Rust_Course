pub fn run(){
 println!("Running..");

 let age:u32=18;
 let is_adult:bool=false;

 if age>=18 {
  println!("The boy is adult");
 }else if age <=18 && !is_adult{
  println!("He is not adult");
 }
let is_age:bool=if age>=18 {true}else{false};
if is_age{
 println!("Have right to vote");
}else{
 println!("Do not have right to vote");
}
}