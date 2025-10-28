use std::vec;

fn main() {
    
    let num = 50;

    if num > 40 {
        println!("The number is greater than 40")
    } else {
        println!("The number is less than 40")
    }


    let score = 90;


    let grade = if score >= 90 {
        'A'
    } else if score >= 80 {
        'B'
    } else if score >= 70 {
        'C'
    } else {
        'D'
    };

    println!("the grade is {}", grade);

    // match statement

    let score = 50;

    let grade = match score {
        90..=100 => 'A',
        80..=89 => 'B',
        70..=79 => 'C',
        _ => 'F'
    };

    println!("The grade handled with match is {grade}");


    // Loops and loop labels
    'outer: loop {
        loop {
            println!("Simple loop...");
            break 'outer;
        }
    }

    // For loops
    let vec_numbers = vec![12, 4, 5, 8, 9, 10];

    for i in vec_numbers {
        println!("{i}")
    }

    let mut num = 0;

    // While Loops
    while num < 10 {
        num += 1;
    }

    
    println!("The value of number after the loop is {}", num)
}
