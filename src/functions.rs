pub fn run(){
 println!("Running..");
 greeting("Good Morning",  "Ronnie");
 let sum=add(1,1);
 println!("{}",sum);
}

fn greeting(greet:&str,name:&str){
 println!("{} {}",greet,name);
}

fn add(n1:u32,n2:u32)->u32{
 n1+n2
}