use rand::Rng;

pub fn validate(valor: &str) -> bool {
    let numbers = valor
        .chars()
        .filter(|s| !"./-".contains(s.to_owned()))
        .collect::<Vec<_>>();

    if numbers.len() != 14 || equal_digits(&numbers) {
        return false;
    }

    let digit_one = validate_first_digit(&numbers);
    if digit_one != numbers[12].to_string().parse::<usize>().unwrap() {
        return false;
    }

    let digit_second = validate_second_digit(&numbers);
    if digit_second != numbers[13].to_string().parse::<usize>().unwrap() {
        return false;
    }

    return true;
}

pub fn generate() -> String {
    let mut rng = rand::thread_rng();
    const CHARSET: &[u8] = b"0123456789";

    let mut vec: Vec<char> = (0..8)
        .map(|_| {
            let idx = rng.gen_range(0, CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    vec.extend(vec!['0', '0', '0', '1']);
    vec.push(CHARSET[validate_first_digit(&vec)] as char);
    vec.push(CHARSET[validate_second_digit(&vec)] as char);

    return vec.into_iter().collect();
}

fn validate_first_digit(numbers: &Vec<char>) -> usize {
    let array: [usize; 12] = [5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];
    let mut _sum = 0;
    for (position, number) in numbers.iter().enumerate() {
        if position <= 11 {
            let number_int = number.to_string().parse::<usize>().unwrap();
            _sum += number_int * array[position];
        }
    }
    let result = _sum % 11;

    if result < 2 {
        return 0;
    }
    return 11 - result;
}

fn validate_second_digit(numbers: &Vec<char>) -> usize {
    let array: [usize; 13] = [6, 5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];
    let mut _sum = 0;
    for (position, number) in numbers.iter().enumerate() {
        if position <= 12 {
            let number_int = number.to_string().parse::<usize>().unwrap();
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
