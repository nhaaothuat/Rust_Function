fn sum_even_numbers(numbers: &[i32]) -> i32 {
    let mut sum = 0;
    for &num in numbers{
     if num%2==0{
         sum+=num;
      }
   }
   return sum;
}

fn main(){
   let num_1 = [1,2,3,4,5,6];
   println!("{}",sum_even_numbers(&num_1));
   let num_2 = [10,20,30,40,50];
   let num_3 = [15,25,35,45,55];
   let num_4 = [-2,0,2,4,6];
   println!("{}",sum_even_numbers(&num_2));
   println!("{}",sum_even_numbers(&num_3));
   println!("{}",sum_even_numbers(&num_4));
}
