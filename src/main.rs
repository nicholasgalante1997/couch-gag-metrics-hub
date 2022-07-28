use std::cmp::Ordering;

fn main() {
    // loading notes into this package until I'm ready 
    // to apply it all into a http server to pump metrics into

    // Variable Decl
    let _foo = 5; // immutable
    let mut _bar = 22; // mutable

    // String
    let mut _str1 = String::new(); // Creating a string with the new creator

    // println! placeholders
    println!("Variable: {}", "I'm the variable");

    match_demo();
    loop_demo();
    type_annotation_demo();
}

fn match_demo() {
    let x = 22;
    let y = 19;
    match x.cmp(&y) {
        Ordering::Less => println!("x is less than y"),
        Ordering::Greater => println!("y is less than x"),
        Ordering::Equal => println!("x == y")
    }
}

fn loop_demo() {
    let mut x = 0;
    loop {
       x = x + 1;
       println!("{}", x);
       if x == 20 {
        break;
       }
    }
}

fn type_annotation_demo() {
    
}