pub fn run(){
    let mut numbers: Vec<i32> = vec![1,2,3,4,5];
    numbers.push(32);
    numbers.push(3);
    numbers.pop();
    println!("{:?}",numbers);
    numbers[2]= 20;
    println!("{}",numbers.len());

    println!("{:?}",numbers[0]);
    let slise: &[i32] = &numbers[0..2];
    println!("{:?}",slise);
    for x in numbers.iter() {
        println!("{}",x);
    }
    for x in numbers.iter_mut() {
        *x *=2
    }

    println!("{:?}",numbers);
}