use std::time::{Duration,Instant};
fn main() {
    let start=Instant::now();
    let n=100000;
    print_n(n);
    let duration=start.elapsed();
    println!("Time elapsed in print_n function  is: {:?}", duration);
}
fn print_n(n:i32){
    for i in 0..=n{
       println!("{}",i);
    }
}