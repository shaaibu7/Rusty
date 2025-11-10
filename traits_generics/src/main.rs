use std::ops::Mul;

struct Square<T> {
    side: u64,
    dimension: T
}

trait ShapeTraits {
    fn area(&self) -> u64;
    fn perimeter(&self) -> u64;
}

impl<T> ShapeTraits for Square<T> where T: Copy + Clone + Mul {
    fn area(&self) -> u64 {
        self.side * self.side
    }

    fn perimeter(&self) -> u64 {
        self.side * 4
    }
}

fn main() {
    let square = Square{side: 5, dimension: 2};

    let square_area = square.area();
    let square_perimeter = square.perimeter();

    let _ = square.dimension;

    println!("The area of the rectangle is {} and the perimeter for the rectangle is {}", square_area, square_perimeter);
}
