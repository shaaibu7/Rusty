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
}
