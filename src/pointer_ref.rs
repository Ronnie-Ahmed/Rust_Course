pub fn run(){
 let val:Vec<u32>=vec![1,2,3];
 let val2:&Vec<u32>=&val;
 println!("{:?}",(&val,val2));
}