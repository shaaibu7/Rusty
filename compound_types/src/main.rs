use core::num;

fn main() {
   // Strings
   let fixed_str: &'static str = "Fixed length string";
   let mut flexible_str = String::from("This is a dynamic string");


   // Arrays - Fixed size holding values of same type
   let mut array_1 = [4, 5, 6, 7, 8, 9];
   let num  = array_1[1];

   let array_2 = [0; 10];

   // Vectors - Dynamic sized holding values of same type
   let vec_1 = vec![4, 5, 6, 7, 8, 9];
   let num = vec_1[2];

   // Tuples - hlod values of different types
   let my_info = ("Salary", 40000, "Age", 40);
   let salary_value = my_info.1;

   let (salary, salary_value, age, age_value) = my_info;

   // Empty Tuple
   let unit = ();

   
}
