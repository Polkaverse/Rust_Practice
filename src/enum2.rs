#[derive(Debug)]
enum information {
    Int(i32),
    Float(f64),
    Text(String),
}
pub fn enum2(){


    let row = vec![
        information::Int(59),
        information::Text(String::from("Man in blue")),
        information::Float(100.45),
    ];
    println!("{:?}", row);

}