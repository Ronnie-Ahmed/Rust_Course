// use std::string;

pub fn run(){
  panic!("THis is Number one bullshit");
 // println!("Data Types: ");
 let num:i32=2566;
 let val:f32=2.25365;
 let cha: char='a';
 let is_true:bool=10>55;
  let name:&str="Ronnie";

 println!("{:?}",(num,val,cha,is_true,name ));
 println!("u32 max: {}",std::u32::MAX);

 let nu="889".parse::<i32>().unwrap();
 let _sum=nu+11;
 println!("{}",nu);


}