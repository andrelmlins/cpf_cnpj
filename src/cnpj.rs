use rand::Rng;

pub fn validate(valor: &str) -> bool {
    let numbers: Vec<usize> = match valor
        .chars()
        .filter(|c| !"./-".contains(*c))
        .map(|c| c.to_digit(10).map(|d| d as usize))
        .collect::<Option<Vec<_>>>()
    {
        Some(nums) => nums,
        None => return false, // Found non-digit character
    };

    if numbers.len() != 14 || equal_digits(&numbers) {
        return false;
    }

    let digit_one = validate_first_digit(&numbers);
    if digit_one != numbers[12] {
        return false;
    }

    let digit_second = validate_second_digit(&numbers);
    if digit_second != numbers[13] {
        return false;
    }

    true
}

pub fn generate() -> String {
    let mut rng = rand::thread_rng();

    let mut vec: Vec<usize> = (0..8)
        .map(|_| rng.gen_range(0, 10))
        .collect();

    vec.extend(vec![0, 0, 0, 1]);
    vec.push(validate_first_digit(&vec));
    vec.push(validate_second_digit(&vec));

    vec.into_iter()
        .map(|d| char::from_digit(d as u32, 10).unwrap())
        .collect()
}

fn validate_first_digit(numbers: &[usize]) -> usize {
    const WEIGHTS: [usize; 12] = [5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];

    let sum: usize = numbers
        .iter()
        .take(12)
        .zip(WEIGHTS.iter())
        .map(|(n, w)| n * w)
        .sum();

    let result = sum % 11;
    if result < 2 { 0 } else { 11 - result }
}

fn validate_second_digit(numbers: &[usize]) -> usize {
    const WEIGHTS: [usize; 13] = [6, 5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];

    let sum: usize = numbers
        .iter()
        .take(13)
        .zip(WEIGHTS.iter())
        .map(|(n, w)| n * w)
        .sum();

    let result = sum % 11;
    if result < 2 { 0 } else { 11 - result }
}

fn equal_digits(numbers: &[usize]) -> bool {
    numbers.windows(2).all(|w| w[0] == w[1])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_validate_valid_cnpj() {
        assert!(validate("11222333000181"));
    }

    #[test]
    fn should_validate_valid_cnpj_with_formatting() {
        assert!(validate("11.222.333/0001-81"));
    }

    #[test]
    fn should_reject_invalid_cnpj() {
        assert!(!validate("11222333000182"));
    }

    #[test]
    fn should_reject_cnpj_with_all_same_digits() {
        assert!(!validate("11111111111111"));
    }

    #[test]
    fn should_reject_masked_cnpj_without_panic() {
        assert!(!validate("**.222.333/0001-**"));
    }

    #[test]
    fn should_reject_cnpj_with_letters() {
        assert!(!validate("11222333000AB1"));
    }

    #[test]
    fn should_reject_cnpj_with_special_characters() {
        assert!(!validate("11222333@00181"));
    }

    #[test]
    fn should_generate_valid_cnpj() {
        let cnpj = generate();
        assert!(validate(&cnpj));
    }
}
