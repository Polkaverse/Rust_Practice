mod factorial;
mod fibonacci;
mod bubble_sort;
mod Selection;
mod insertion;


fn main() {
    print!(" Factorial of 5 : ", );
    factorial::fact();

    print!(" Fibonacci series : ", );
    fibonacci::fab();


    print!(" \n Bubble sort : ");
    bubble_sort::bub();

    print!(" \n Insertion Sort : ");
    insertion::ins();

    print!(" \n Selection Sort : ");
    Selection::sel();
}





/*
//mod print;
mod varcon;

fn main() {
    println!("Starting Pankaj chaudhary");
    //print::run();
    varcon::con();
}
*/
