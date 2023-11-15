pub fn run(){
 print!("Running..\n");

 let mut count:u32=0;
 loop {
     count+=1;
     if count>100 {
      break;
     }
     else if count % 5==0 && count%3==0 {
        println!("FizzBuzz");
     }
    else if count%3==0{
        println!("Fizz");
     }else if count%5==0 {
        println!("Buzz");
     }else{
        println!("{}",count);
     }
 }
 count=0;

 while count<=100{
   if count % 5==0 && count%3==0 {
        println!("FizzBuzz");
     }
    else if count%3==0{
        println!("Fizz");
     }else if count%5==0 {
        println!("Buzz");
     }else{ 
        println!("{}",count);
     }
     count+=1;

 }
 for i in 0..100{

   if i % 5==0 && i%3==0 {
        println!("FizzBuzz");
     }
    else if i%3==0{
        println!("Fizz");
     }else if i%5==0 {
        println!("Buzz");
     }else{
        println!("{}",i);
     }

 }
}