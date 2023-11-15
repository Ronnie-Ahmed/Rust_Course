pub fn run(){
 println!("Running");
 let val:[i32;5]=[1,2,3,4,5];
 println!("{:?}",val);
 println!("{}",val[3]);
 let slice:&[i32]=&val[0..2];
 println!("{:?}",slice);
 println!("Size is {}",std::mem::size_of_val(&val));
}