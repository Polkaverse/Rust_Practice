#[macro_use]
extern crate text_io;

mod factorial;
mod fibonacci;
mod bubble_sort;
mod Selection;
mod insertion;
mod slice;
mod structure;
mod method;
mod enums;
mod iflet;
mod packmod;
mod vector;
mod enum2;
mod string;
mod hashmap;
mod errorhand;
mod guessgame;
mod duplicate;
mod generics;
mod generic2;

fn main() {

    print!("\n\nGuess Game  :\n");
    guessgame::guess();


    print!("\n\nAvoid Duplicasy Code   :\n");
    duplicate::dupl();

    print!("\n\nError Handling  :\n");
    errorhand::errorhand();

    print!("\n\nGenerics  :\n");
    generics::generic();

    print!("\n\nGenerics 2 :\n");
    generic2::traitt();



    /*print!(" Factorial of 5 : ", );
    factorial::fact();

    print!(" Fibonacci series : ", );
    fibonacci::fab();


    print!(" \n Bubble sort : ");
    bubble_sort::bub();

    print!(" \n Insertion Sort : ");
    insertion::ins();

    print!(" \n Selection Sort : ");
    Selection::sel();

    print!(" \n Slicing : ");
    slice::sli();

    print!(" \nStructure :  \n");
    structure::struc();

    print!(" \nMethod :  \n");
    method::met();

    print!(" \nEnums :  \n");
    enums::enms();

    print!("iflet :\n");
    iflet::iflet();

    print!("\n\nPackages Crates And Modules : ");
    packmod::pcm();

    print!("\n\nVectors: ");
    vector::vec();

    print!("\n\nEnumerations with multiple types: ");
    enum2::enum2();

    print!("\n\nString : ");
    string::string1();

    print!("\n\nHashmap  :\n");
    hashmap::hash();*/

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
