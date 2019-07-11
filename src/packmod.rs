pub fn pcm() {
    use deeply::nested::function as other_function;

    other_function();

    println!("1st  block");


    function();

    fn function() {
        println!("Second function()");
    }

    mod deeply {
        pub mod nested {
            pub fn function() {
                println!("\nMain funtion which we ant to call");
            }
        }
    }
    deeply::nested::function();
}