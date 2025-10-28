fn print_string(str: &str) {
    println!("{}", str)
}

// function that returns one value of type u8
fn return_data(num: u8) -> u8 {
    num * num
}

// function that returns a tuple
fn return_tuple(num1: i8, num2: i8) -> (i8, i8, i8) {
    (num1 + num2, num1 * num2, num1 - num2)
}

// mutability of function parameters

fn mutate_parameter(mut x: i32) -> i32 {
    x = 8;
    x
}

fn mutability_with_shadowing(x: i32) -> i32{
    let mut x = x;
    x = 7;
    x
}

fn main() {
    let name = "suleiman";
    print_string(name);

    let num = return_data(12);

    println!("{num}");

    let basic_math = return_tuple(12, 4);

    println!("{basic_math:?}");

    let fullname = {
        let first_name = "shaaibu";
        let last_name = "suleiman";
        format!("{first_name} {last_name}")
    };

    println!("{:?}", fullname);

    let mut num = 12;
    let data = mutate_parameter(num);

    println!("{}", data);

    let data = mutability_with_shadowing(23);

    println!("{}", data)
}
