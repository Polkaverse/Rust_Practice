pub fn met() {
    let small = Rectangle {
        width: 8,
        height: 24,
    };

    // rectangle's area
    println!("width is {} height is {} \nAnd area of Rectangle is {}", small.width, small.height, small.area());


    struct Rectangle {
        width: u32,
        height: u32,
    }

    //Area of a rectangle by METHOD

    impl Rectangle {
        fn area(&self) -> u32 {

            //use the . operator to fetch the value of a field via the self keyword
            self.width * self.height
        }
    }
}