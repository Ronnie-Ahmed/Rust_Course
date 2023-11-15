pub fn run(){
 println!("Running");
 let mut val:Vec<u32>=vec![1,2,5,8];
 println!("{:?}",&val);
 val.push(3);
 println!("{:?}",&val);
 val.pop();
 println!("{:?}",&val);
 println!("Size of the Vector: {}",std::mem::size_of_val(&val));
 val.push(100);
 println!("{}",std::mem::size_of_val(&val));
 for i in 1..11{
  val.push(i);

 }
  println!("{:?}",&val);
  let im:u32=2;
  for i in 0..val.len(){
   if val[i]==im{
    val[i]=val[val.len()-1];
    val.pop();
    break;
   }

  }
  println!("{:?}",&val);

  for i in val.iter_mut(){
   *i *=2;
   println!("{}",i);
  }



}