pub fn validate(valor: &str) -> bool {
    if valor.chars().count() != 14 {
        return false;
    }
    let numbers: Vec<&str> = valor.split("").collect();

    // if equal_digits(&numbers) {
    //     return false;
    // }

    let digit_one = validate_first_digit(&numbers);
    if digit_one != numbers[13].parse::<i16>().unwrap() {
        return false;
    }

    let digit_second = validate_second_digit(&numbers);
    println!("{}", digit_second);
    if digit_second != numbers[14].parse::<i16>().unwrap() {
        return false;
    }

    return true;
}

fn validate_first_digit(numbers: &Vec<&str>) -> i16 {
    let array: [i16; 12] = [5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];
    let mut _sum = 0;
    for (position, number) in numbers.iter().enumerate() {
        if number.to_owned() != "" && position <= 12 {
            let number_int = number.parse::<i16>().unwrap();
            _sum += number_int * array[position - 1];
        }
    }
    let result = _sum % 11;

    if result < 2 {
        return 0;
    }
    return 11 - result;
}

fn validate_second_digit(numbers: &Vec<&str>) -> i16 {
    let array: [i16; 13] = [6, 5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];
    let mut _sum = 0;
    for (position, number) in numbers.iter().enumerate() {
        if number.to_owned() != "" && position <= 13 {
            let number_int = number.parse::<i16>().unwrap();
            _sum += number_int * array[position - 1];
        }
    }
    let result = _sum % 11;

    if result < 2 {
        return 0;
    }
    return 11 - result;
}

fn equal_digits(numbers: &Vec<&str>) -> bool {
    let mut digit = "";

    for number in numbers {
        if number.to_owned() != "" {
            if digit == "" {
                digit = number;
            } else if digit != number.to_owned() {
                return false;
            }
        }
    }

    return true;
}
