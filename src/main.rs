fn main() {
    for n in 1..100 {
        println!("{}", generate_fizz_buzz(n));
    }
}

pub fn generate_fizz_buzz(_n: i32) -> &'static str {
    return if is_divisible_by_three(_n) && is_divisible_by_five(_n) {
        "FizzBuzz"
    } else if is_divisible_by_five(_n) {
        "Buzz"
    } else if is_divisible_by_three(_n) {
        "Fizz"
    } else { "" };
}

fn is_divisible_by_three(_number: i32) -> bool {
    return _number % 3 == 0;
}

fn is_divisible_by_five(_number: i32) -> bool {
    return _number % 5 == 0;
}

#[cfg(test)]
mod tests {
    #[test]
    fn when_divisible_by_3_and_5_return_fizz_buzz() {
        assert_eq!("FizzBuzz", super::generate_fizz_buzz(15));
    }

    #[test]
    fn when_divisible_by_5_return_buzz() {
        assert_eq!("Buzz", super::generate_fizz_buzz(5));
    }

    #[test]
    fn when_divisible_by_3_return_fizz() {
        assert_eq!("Fizz", super::generate_fizz_buzz(3));
    }
}