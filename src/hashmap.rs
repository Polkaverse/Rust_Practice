pub fn hash(){
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 221);
    scores.insert(String::from("Black"), 239);

    scores.insert("Yellow".parse().unwrap(), 236);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("{:?}",score);


    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }


    scores.insert(String::from("Blue"), 225);

    println!("{:?}", scores["Blue"]);


    scores.entry(String::from("Blue")).or_insert(50);  // Value will not be inserted
    println!("{:?}", scores["Blue"]);
}

