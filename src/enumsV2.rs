use std::{io::{self, Read}, convert};

#[derive(Debug)]
enum Ip_Addr{
 V4(u8,u8,u8,u8), 
 V6(String),
}
enum option<T>{
 None,
 Some(T)
}
struct Ipinfo{
 kind:Ip_Addr,
 address:String
}
impl Ipinfo {
fn getinfo(&self)->String{
 format!("`address : {}  Kind: {:?}`",self.address,self.kind)
}
    
}

enum Coin{
 Penny,
 Nikel,
 Dinhar,
 Dime,
 Quarter,
}


pub fn run(){
 println!("Runing...");
 // let state=Ip_Addr::V4;
 // println!("{:?}",state);
 let homeAddress:Ipinfo=Ipinfo { kind: Ip_Addr::V4(127,0,0,1), address: "127.0.0.1".to_string() };
 println!("{:?}",homeAddress.getinfo());
 let someNum=Some(5);
 let some_chars=Some("A");
 println!("{:?}",someNum);
 let mut co:String=String::new();
 io::stdin().read_line(&mut co).expect("Enter a right Dime");
 
 let coin:Coin=Coin::Dime;
match coin{
 
  Coin::Dime=>println!("Dime"),
  Coin::Dinhar=>println!("Dinhar"),
  Coin::Nikel=>println!("Nikel"),
  Coin::Penny=>println!("Penny"),
  Coin::Quarter=>println!("Quarter")
     
 
}

let num =9;
match num{
 9=>println!("Number one"),
 2=>println!("Number two"),
 _=>println!("asasd")
}
}

fn route(situation:Ip_Addr){

}