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





}