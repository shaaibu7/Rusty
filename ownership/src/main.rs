fn takes_ownership(vec: Vec<i32>) {
    println!("vec is: {:?}", vec)
}

fn gives_ownership() -> Vec<i32> {
    vec![12, 34, 56]
}

fn takes_and_gives_ownership(mut vec: Vec<i32>) -> Vec<i32> {
    vec.push(10);
    vec
}

fn function_that_takes_stack_data(mut data: i32) {
    data = 7;
    println!("The stack data is: {}", data);
}

fn immutable_borrow(vec: &Vec<i32>) {
    println!("The immutably borrowed vector is: {:?}", vec);
}

fn mutable_borrow(vec: &mut Vec<i32>) {
    vec.push(12);
}

fn mixed_borrows(subject: &String, score: &mut Vec<i32>) {
    println!("The subject {} and score before {:?}", subject, score);
    score.push(7);
    println!("The subject {} and score after {:?}", subject, score);

}
fn main() {
    let vec = vec![12, 34, 87, 0];
 
    // Function that takes ownership
    takes_ownership(vec);
    

    // Function that returns ownership
    let vec_2 = gives_ownership();

    // Function that take and return ownership
    let vec_3 = takes_and_gives_ownership(vec_2); 

    let x = 87;

    function_that_takes_stack_data(x);

    println!("The data is still valid and not moved {}", x);


    // Borrowing

    let vec_ref = vec![1, 2, 3, 4, 5, 6];

    // Immutable borrow
    let vec_ref1 = &vec_ref;
    let vec_ref2 = &vec_ref;

    // Dereferrencing of stack and heap allocated data

    // Dereferencing stack allocated data
    let mut data = 45;
    let ref_1 = &mut data;
    // data is copied
    let deref_copy = *ref_1;
    println!("The value of deref copy is: {}", deref_copy);
    *ref_1 = 7;
    println!("The value os deref_copy is: {}", deref_copy);
    println!("The value of data is: {}", data);

    // Dereferencing heap allocated data
    let mut data = vec![1, 2, 3, 4, 5, 6];
    let ref_1 = &mut data;

    ref_1.push(7);
    // *ref_1.push(7); Wrong way of derefencing and updating heap allocated data
    (*ref_1).push(7);

    println!("The reference data is {:?}", ref_1);

    let tuple = (String::from("suleiman"), 8);

    let (ref name, age) = tuple;

    println!("The data is: {}", tuple.0)
}
