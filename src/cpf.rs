pub fn validate(valor: &str) -> bool {
    if valor.chars().count() != 11 {
        return false;
    }
    let numbers: Vec<&str> = valor.split("").collect();

    let digit_one = validate_first_digit(&numbers);
    if digit_one != numbers[10].parse::<i16>().unwrap() {
        return false;
    }

    let digit_second = validate_second_digit(&numbers);
    if digit_second != numbers[11].parse::<i16>().unwrap() {
        return false;
    }

    return true;
}

fn validate_first_digit(numbers: &Vec<&str>) -> i16 {
    let mut count = 10;
    let mut _sum = 0;
    for number in numbers {
        if number.to_owned() != "" && count >= 2 {
            let number_int = number.parse::<i16>().unwrap();
            _sum += number_int * count;
            count -= 1;
        }
    }
    let result = (_sum * 10) % 11;

    if result == 10 {
        return 0;
    } else {
        return result;
    }
}

fn validate_second_digit(numbers: &Vec<&str>) -> i16 {
    let mut count = 11;
    let mut _sum = 0;
    for number in numbers {
        if number.to_owned() != "" && count >= 2 {
            let number_int = number.parse::<i16>().unwrap();
            _sum += number_int * count;
            count -= 1;
        }
    }
    let result = (_sum * 10) % 11;

    if result == 10 {
        return 0;
    } else {
        return result;
    }
}
