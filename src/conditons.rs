pub fn run(){
    let age: u8 = 18;
    let check_id: bool = false; 
    let mutad = true;
    if age >= 21 && check_id || mutad{
        println!("vad vill du drika");
    }else if age <21 && check_id{
        println!("sory kido")
    }else {
        println!("i haw to check your id")
    }
    let is_of_age = if age >=21 {true}else{false};
    println!("{}",is_of_age)
}