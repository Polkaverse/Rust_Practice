pub fn vec(){
    let mut v=Vec::new();
    v.push('p');
    v.push('a');
    v.push('n');
    v.push('k');
    v.push('a');
    v.pop();
    v.push('j');
    v.push(' ');
    v.push('c');
    v.push('h');
    for i in v
        {
            print!("{}",i);
        }

}