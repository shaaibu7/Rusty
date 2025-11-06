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
}