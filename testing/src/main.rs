enum Gender {
    Male,
    Female
}


struct Student {
    name: String,
    age: u8,
    gender: Gender,
    height: f64
}

impl Student {
    fn get_student_name(&self) -> &String {
        let name = &self.name;
        name
    }
}

fn main() {
    println!("Hello, world!");
}

fn addtion(num1: u64, num2: u64) -> u64 {
    num1 + num2
}

fn append_data(data: &mut Vec<u8>, item: u8) {
    data.push(item);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition() {
        let result = addtion(45, 5);
        assert_eq!(result, 50);
    }

    #[test]
    fn test_append() {
        let mut numbers = vec![1, 2, 3, 4];
        append_data(&mut numbers, 5);
        assert_eq!(numbers.len(), 5)
    }

    #[test]
    fn test_get_student() {
        let student = Student{name: String::from("suleiman"), age: 17, gender: Gender::Male, height: 98.1};
        let student_name = student.get_student_name();

        assert_eq!(student_name, "suleiman")
    }
}