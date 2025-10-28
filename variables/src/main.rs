fn main() {
    // variables are immutable in rust by default
    let num = 12; 

    // you can add the mut keyword to make variables mutable
    let mut data = 12;

    data = 67;

    println!("The mutated number is {}", data);

    // scopes and how they work
    {
        // only valid in this scope
        let x = 67;
    }

    // shadowing
    let x = 7;
    let x = x + 7;
    println!("The new value of x because it has been shadowed is {}", x);


    // constant 
    const PI: f64 = 3.142; // type for constant must be annotated, it cannot be inferred
    println!("The value of PI {}", PI)
   
}
