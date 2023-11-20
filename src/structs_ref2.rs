struct Person{
 isAdult:bool,
 firstName:String,
 lastName:String,
 age:u32,
}

struct Curve{
 width:u32,
 length:u32,
 height:u32,
}

impl Curve {
 fn area(&self)->u32{
  self.height*self.length*self.width
 }
 fn width(&self)->bool{
  self.width>0
 }
 fn can_hold(&self, other:&Curve)->bool{
 self.width> other.width && self.length > other.length && self.height > other.height
 }
    
}

struct Rectangle{
 width:u32,
 height:u32,
}
pub fn run(){
 println!("RUnning>>>>");
 let person=Person{isAdult:true,firstName:"Ronnie".to_string(),lastName:"Ahmed".to_string(),age:13};
 println!("{:?}",(person.firstName,person.lastName));
 let rect=Rectangle{width:32,height:32};
 let ar=area(&rect);
 let ar2=area2(rect.height, rect.width);
 println!("Area is {ar}");
 println!("Area is {ar2}");

 let curve=Curve{width:32,length:32,height:32};
 let other=Curve{width:31,length:31,height:31};
 println!("Area of curve is {}",curve.area());

 if curve.width(){
  println!("The area width is : {}",rect.width);
 }
 println!("can Hold {}",curve.can_hold(&other));

}
fn area(rectangle:&Rectangle)->u32{
 rectangle.width*rectangle.height
}

fn area2(height:u32,widht:u32)->u32{
 height*widht
}