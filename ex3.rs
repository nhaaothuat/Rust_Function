fn calculate_average(numbers: &[f64]) -> f64 {
    let sum: f64 = numbers.iter().sum();
    let count = numbers.len() as f64;
    return  sum/count; 
}
fn main(){
  let number_1 = [2.5,4.8,6.3,1.7,3.9];
  
  println!("{}",calculate_average(&number_1));
  let number_2 = [];
  println!("{}",calculate_average(&number_2));
}
