#[derive(Debug)]
struct Example1<T>{
 x:T,
 y:T,
}
struct Example2<T,U>{
 x:T,
 y:U,
}

impl <T> Example1<T>{
fn show(&self)->&T{
 &self.x
}

}


pub fn run(){
 println!("Running");
 let test1:Example1<i32>=Example1 { x: 12, y: 12 };
 let test2:Example1<f32>=Example1 { x: 12.05, y: 12.23 };
 let test3:Example1<char>=Example1 { x: 'A', y: 'B' };
 let test4:Example2<i32,f32>=Example2 { x: 12, y: 12.5 };
 let test5:Example2<&str,String>=Example2 { x: "Ronnie", y: String::from("ROnnie") };
 println!("{} {}",test1.x,test1.y);
 println!("{} {}",test2.x,test2.y);
 println!("{} {}",test3.x,test3.y);
 println!("{} {}",test4.x,test4.y);
 println!("{} {}",test5.x,test5.y);
 let test6:Example1<i32>=Example1 { x: 12, y: 12 };
 println!("{:?}",test6.show());
}