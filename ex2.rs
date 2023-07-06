pub fn sum_one_to_n(n: u32) -> u32{
   let mut sum =0;
   
   for i in 1..=n{ sum+=i; }
   return sum;
}

fn main(){
   println!("{}",sum_one_to_n(0));
    println!("{}",sum_one_to_n(1));
    println!("{}",sum_one_to_n(100));
}
