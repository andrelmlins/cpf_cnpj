pub fn validate(valor: &str) -> bool {
    let numbers = valor
        .chars()
        .filter(|s| !"./-".contains(s.to_owned()))
        .collect::<Vec<_>>();

    if numbers.len() != 14 || equal_digits(&numbers) {
        return false;
    }

    let digit_one = validate_first_digit(&numbers);
    if digit_one != numbers[12].to_string().parse::<i16>().unwrap() {
        return false;
    }

    let digit_second = validate_second_digit(&numbers);
    if digit_second != numbers[13].to_string().parse::<i16>().unwrap() {
        return false;
    }

    return true;
}

fn validate_first_digit(numbers: &Vec<char>) -> i16 {
    let array: [i16; 12] = [5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];
    let mut _sum = 0;
    for (position, number) in numbers.iter().enumerate() {
        if position <= 11 {
            let number_int = number.to_string().parse::<i16>().unwrap();
            _sum += number_int * array[position];
        }
    }
    let result = _sum % 11;

    if result < 2 {
        return 0;
    }
    return 11 - result;
}

fn validate_second_digit(numbers: &Vec<char>) -> i16 {
    let array: [i16; 13] = [6, 5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];
    let mut _sum = 0;
    for (position, number) in numbers.iter().enumerate() {
        if position <= 12 {
            let number_int = number.to_string().parse::<i16>().unwrap();
            _sum += number_int * array[position];
        }
    }
    let result = _sum % 11;

    if result < 2 {
        return 0;
    }
    return 11 - result;
}

fn equal_digits(numbers: &Vec<char>) -> bool {
    let mut digit = String::from("");

    for number in numbers {
        if digit == "" {
            digit = number.to_string();
        } else if digit != number.to_string() {
            return false;
        }
    }

    return true;
}
