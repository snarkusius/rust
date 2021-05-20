
pub fn run(){
    let hello = "hello";
    let mut hello2 = String::from("hello ");
    //get length
    println!("Length: {}", hello2.len());

    //lägga på en bokstav 
    hello2.push('w');
    //lägga på en string
    hello2.push_str("orld");
    //capasaty
    println!("cappasety: {}",hello2.capacity());
    //contains
    println!("contains world {}",hello2.contains("world"));
    //replase
    println!("repase: {}", hello2.replace("world","ther"));

    println!("{}",hello2);
    for word in hello2.split_whitespace(){
        println!("{}",word);
    }
    //bestäm kapasety
    let mut s = String::with_capacity(10);
    s.push('A');
    s.push('b');
    println!("{}",s);

    assert_eq!(2, s.len());
    assert_eq!(10,s.capacity());
    

}