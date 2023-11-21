pub fn run(){
 let mut num:Vec<u32>=Vec::new();
 for i in 1..101{
  num.push(i);
 }
 for i in num.iter(){
  print!("{} ",i);
 }
}