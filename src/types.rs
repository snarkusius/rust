
pub fn run(){
    let x = 1;
    //i32
    let y = 2.5;
    //f64
    let z: i64 = 4000000000030;
    let is_activ: bool = true;
    let is_greater: bool = 10> 5 ;
    let a1 = 'a';
    let face = '\u{1F600}';
    println!("{:?}",(x,y,z,is_activ, is_greater,a1,face));
    println!("max i32:{}", std::i32::MAX);
    println!("max i64:{}", std::i64::MAX);
}