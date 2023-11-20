#[allow(unused_variables)]
use std::{io::{self, Read}, convert};

pub fn run(){
 println!("Running...");
 // let mut guess:String=String::new();
 // io::stdin().read_line(&mut guess).expect("Failed to Read the line");
 // println!("Your name is {}",guess);

 let num:u32=123;
 {
  let num: u32=num+123;
  println!("Number is : {num}");
 }
 println!("Number is : {num}");
let spaces:&str="   ";
println!("Space Length: {}",spaces.len());
let _x:u32=654654;
let guess:u32="42".parse::<u32>().unwrap();
println!("Number is : {}",guess+2);

let tup:(u32,i32,f32,bool,&str)=(23,23,4.2,true,"asd");
let x:&str=tup.4;
println!("{}",x);

// let a:[u32;5]=[1,2,3,4,5];
let mut b:Vec<u32>=vec![2,3,5,5,67,8];

println!("Please enter the index number: ");
let mut index:String=String::new();
io::stdin().read_line(&mut index).expect("Not a number");
let index: usize=index.trim().parse::<usize>().expect("Not a numer");
println!("Enter a new number: ");
let mut enterValue:String=String::new();
io::stdin().read_line(&mut enterValue).expect("Not a number");
let mut enter_value=enterValue.trim().parse::<u32>().expect("not a number");
b.push(enter_value);

println!("{}",b[index]);
println!("{:?}",b);

let va:u32=123;
let vaa:i32=va as i32;
println!("{}",vaa);
println!("{},{}",i8::MAX,u8::MAX);
 let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
 println!("{}",v);

 let c1:char='中';
 let size_of:usize=std::mem::size_of_val(&c1);
 println!("{}",size_of);

 let y:i32={
  let mut x:i32=5;
   x+10+29


 };
let mut count=0;
 loop {
     println!("RUnning...");
    count+=1;
    if count==100 {break;}

 }




}
