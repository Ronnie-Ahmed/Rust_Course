use std::collections::HashMap;

#[derive(Debug)]
enum SpreadSheetCall{
 Int(u32),
 Float(f32),
 Text(String),
}

pub fn run(){
 let mut arr: Vec<u32>=Vec::new();
 println!("{:?}",arr);
 arr.push(23);
 println!("{:?}",arr);
 for i in 1..=100{
  arr.push(i);
 }

 let ar: u32=arr[29];
 println!("ar: {}",ar);
 println!("{:?}",arr);

 let row=vec![
  SpreadSheetCall::Int(45),
  SpreadSheetCall::Float(2.3),
  SpreadSheetCall::Text("RONNIE".to_string())
 ];
 println!("{:?}",row);

 let mut s:String=String::new();
 s.push_str("ROnNIE");
 s.push_str(" AHmed");
 println!("Name is : {:?}",s);
 let ss:String=s.clone();
 println!("Name is : {:?}",s);

 ///////////////
 /// HashMap////
 ///////////////

 let mut map:HashMap<u32, String>=HashMap::new();
 for i in 1..=100{
  if i%5==0{
   map.insert(i, "Ronnie".to_string());
  }
 }
 // map.insert(10, "Ronnie".to_string());
 println!("{:?}",map.get(&15));
 let arr = [1, 2, 3];
 let v1=Vec::from(arr);
 println!("{:?}",v1);
 println!("{:?}",arr);
 let v2:Vec<i32>=arr.into();
 println!("{:?}",v2);

 let st:String=String::from("Ronnie");
 let v3=Vec::from(st);
  println!("{:?}",v3);

   let mut v = Vec::from([1, 2, 3]);
    for i in 0..5 {
        println!("{:?}", v.get(i))//option
    }

    for i in 0..5 {
       // IMPLEMENT the code here...
       
       match v.get(i){
           Some(e)=>v[i]=e+1,
           None=>v.push(i+2)
       }
    }
    
    assert_eq!(v, vec![2, 3, 4, 5, 6]);

    println!("Success!");

     let mut scores= HashMap::with_capacity(100);
    scores.insert("Sunface", 98);
    scores.insert("Daniel", 95);
    scores.insert("Ashley", 69.0 as i32);
    // scores.insert("Katie", "58".parse().);
    // println!("{:?}"scores.get(&"Katie"));
    println!("{}",scores["Ashley"]);
    scores.remove("Ashley");
    println!("Length is : {}",scores.len());
    println!("Length is : {}",scores.capacity());
    scores.shrink_to_fit();
    println!("Length is : {}",scores.capacity());

    let decimal:f32=97.3364;
    let integer:i32=decimal as i32;
    let unsigned_integer:u8=integer as u8;
    let chat_type:char=unsigned_integer as char;
    println!("{}",chat_type);

    let testString:String="Ronnie".to_string();
    let testStr:&str=&testString.as_str();
    // let owned_string = String::from("Hello, Rust!");
    // let borrowed_str: &str = owned_string.as_str();
    // println!("Owned String: {}", owned_string);
    // println!("Borrowed &str: {}", borrowed_str);

    println!("{}",testString);
    println!("{}",testStr);


    let nu:u32=5.256 as u32;
   







}