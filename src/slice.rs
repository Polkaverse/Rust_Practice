pub fn sli() {
    let str: String = read!("{}\n");
    let stri = String::from(str);
    let pcs = &stri[..=6];
    println!("\n first word of the given string is {}", pcs);


    //let str= String::from("hello world") ;
    let hello = &stri[0..3];
    let world = &stri[6..];
    println!(" {}  {} ", hello, world);
    let p =first_word(& stri);
    print!("{}" , p);
}


fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}