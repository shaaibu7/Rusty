fn main() {
    // unsigned integers
    let unsigned_num: u8 = 5; // u16, u32, u64, u128

    // Signed integers
    let signed_num: i8 = 5; // i16, i32, i64, i128

    //floating point numbers
    let float_num: f32 = 5.0; //f64

  
   // Platform specific integers are used for indexing collections such as arrays and vectors
   let arch1: usize = 5;
   let arch2: isize = 5;

   // characters
   let char: char = 'a';

   // Boolean
   let b: bool = true;

   // Types Aliasing
   type Age = u8;
   let age: Age = 67;

   // Type conversion
   let PI = 3.142 as f64;

}
