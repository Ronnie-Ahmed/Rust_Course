// // use std::arch::x86_64::__get_cpuid_max;

// pub fn run(){
//  println!("Running");
//  let val:[i32;5]=[1,2,3,4,5];
//  println!("{:?}",val);
//  println!("{}",val[3]);
//  let slice:&[i32]=&val[0..2];
//  println!("{:?}",slice);
//  println!("Size is {}",std::mem::size_of_val(&val));
//  // let sum:String=addvalue("12".to_string(),"23".to_string());
//  // println!("{}",sum);

//  let n1="12".as_bytes();
//  let n2="121".as_bytes();
//  let v:usize=n2.len().max(n1.len());

//  println!("Length is  :  {}",v);

//  println!("{:?}",n1);
//  let max_value=n1.iter().max();
//  println!("{:?}",n1.iter().max());
//  // println!("{:}",max_value);
//  match max_value{
//   Some(&max)=>println!("{}",max),
//   None=>println!("No value Found")
//  };
// }


// // fn addvalue(num1:String,num2:String)->String{
// //  let _num1:i32=num1.parse().unwrap();
// //  let _num2:i32=num2.parse().unwrap();
// //  let _sum=_num1+_num2;
// //  _sum.to_string()
// // }