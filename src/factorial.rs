pub fn fact() {
    let mut i = 5;
    let mut n = 1;
    while i >= 1 {
        n=n*i;
        i=i-1;
    }
    println!("{}",n);
}