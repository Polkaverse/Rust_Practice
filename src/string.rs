pub fn string1(){
    let s = "Welcome".to_string();
    let u: String = " in this".into();
    let t = String::from(" world.");
    let combined= s+ &u + &t;

    print!("{}",combined);



    for i in combined.chars(){
        print!("{} ",i);
    }


}