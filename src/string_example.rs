pub fn run(){

 let mut name:String=String::from("Ronnie ");
 println!("{}",name);
 name.push('A');
 println!("{}",name);

 name.push_str("hmed");
  println!("{}",name);

  let  is_here:bool=name.contains("Ahmed");
  println!("String length: {}",name.len());
  println!("String Capacity: {}",name.capacity());
  println!("Does Contain: {}",is_here);
  name=name.replace("Ahmed", "Raisul");

  let is_here2: bool=name.contains("Raisul");
    println!("Does Contain: {}",is_here2);

    for i in name.split(" "){
     println!("{}",i);
    }

    let mut l:String=String::with_capacity(12);
    l.push('A');
    l.push('a');
    assert_eq!(l.len(),2);
    assert_eq!(l.capacity(),12);
    println!("{}",l);



 
}