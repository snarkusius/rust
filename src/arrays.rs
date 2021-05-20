pub fn run(){
    let mut numbers:[i32;5] = [1,2,3,4,5];
    println!("{:?}",numbers);
    numbers[2]= 20;
    println!("{}",numbers.len());

    println!("{:?}",numbers[0]);
    let slise: &[i32] = &numbers[0..2];
    println!("{:?}",slise)
}