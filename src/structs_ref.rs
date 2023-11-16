//Traditional Structs

struct Color{
 red:u8,
 green:u8,
 blue:u8
}

struct TupleColor(u8,u8,u8);

struct Person{
 first_name:String,
 last_name:String
}
impl Person {
 fn new(first:&str,last:&str)->Person{
  // format!("First Name:{} Last Name:{}",&first,&last)
  Person { first_name: first.to_string(), last_name: last.to_string() }
 }

 fn full_name(&self)->String{
  format!("{} {}",self.first_name,self.last_name)
 }
 fn set_last_name(&mut self,last:&str){
  self.last_name=last.to_string();
 }

 fn tuple(self)->(String,String){
  (self.first_name,self.last_name)
 }


    
}

pub fn run(){
 let mut color_ref:Color=Color { red: (255), green: (255), blue: (0) };
 println!("red: {}  Green: {}  Blue: {}",color_ref.red,color_ref.green,color_ref.blue);
 color_ref.blue=134;

 println!("{:?}",(color_ref.red,color_ref.blue,color_ref.green));

 let tuple_color:TupleColor=TupleColor(123, 123,123);
 println!("{:?}",(tuple_color.0,tuple_color.1,tuple_color.2));
 let mut person:Person=Person::new("Ronnie", "Ahmed");
 println!("{:?}",(person.first_name,person.last_name));
 // person.set_last_name("Islam");
 println!("{:?}",person.tuple());


 


}