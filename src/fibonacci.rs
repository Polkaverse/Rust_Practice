pub fn fab(){
    let mut i = 0;
    let mut n = 1;
    let mut b=0;
    print!(" 1");
    for p in 0..6{
        print!(" {}",i+n);

        b=n;
        n=i+n;
        i=b;

    }
}