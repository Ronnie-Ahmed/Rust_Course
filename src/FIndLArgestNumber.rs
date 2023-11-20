pub fn run(){

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result=largestNumber(&number_list);
    println!("Largest Number is : {}",result);

}

fn largestNumber(list:&[i32])->&i32{
 let mut largest=&list[0];
 for num in list{
  if largest<num{
   largest=num;
  }
 }
 largest
}