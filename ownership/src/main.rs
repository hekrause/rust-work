//! Ownership examples
fn main() {
    //moved_value();
    //copy_types();
    borrow_value_through_reference();
}

pub fn moved_value() {
//    let word = "text".to_string();
//    let words = word;
//
//    println!("({}", word);
}

pub fn copy_types() {
    let var = 10;
    let value = var;

    println!("Var: {}", var);
    println!("Value: {}", value);
}

pub fn borrow_value_through_reference() {
    let mut count = 0;
    let const_val = 0;

    {
        let mut_ref = &mut count;
        *mut_ref = 2;
    }

    let count_ref = &count;
    let const_ref = &const_val;

    println!("{}", &count_ref);
    println!("{}", &const_ref);
}
