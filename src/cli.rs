pub fn run(){
 let name:Vec<String>=std::env::args().collect();

 println!("{:?}",name);
 let name2:Vec<String>=name.clone();

 for i in name2.iter(){
  if i=="Ronnie"{
   println!("{}",i);
  }
 }
}