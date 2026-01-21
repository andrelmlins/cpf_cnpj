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

    if numbers.len() != 11 || equal_digits(&numbers) {
        return false;
    }

    let digit_one = validate_first_digit(&numbers);
    if digit_one != numbers[9] {
        return false;
    }

    let digit_second = validate_second_digit(&numbers);
    if digit_second != numbers[10] {
        return false;
    }

    true
}

pub fn generate() -> String {
    let mut rng = rand::thread_rng();

    let mut vec: Vec<usize> = (0..9)
        .map(|_| rng.gen_range(0, 10))
        .collect();

    vec.push(validate_first_digit(&vec));
    vec.push(validate_second_digit(&vec));

    vec.into_iter()
        .map(|d| char::from_digit(d as u32, 10).unwrap())
        .collect()
}

fn validate_first_digit(numbers: &[usize]) -> usize {
    let sum: usize = numbers
        .iter()
        .take(9)
        .enumerate()
        .map(|(i, n)| n * (10 - i))
        .sum();

    let result = (sum * 10) % 11;
    if result == 10 { 0 } else { result }
}

fn validate_second_digit(numbers: &[usize]) -> usize {
    let sum: usize = numbers
        .iter()
        .take(10)
        .enumerate()
        .map(|(i, n)| n * (11 - i))
        .sum();

    let result = (sum * 10) % 11;
    if result == 10 { 0 } else { result }
}

fn equal_digits(numbers: &[usize]) -> bool {
    numbers.windows(2).all(|w| w[0] == w[1])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_validate_valid_cpf() {
        assert!(validate("40743854063"));
    }

    #[test]
    fn should_validate_valid_cpf_with_formatting() {
        assert!(validate("407.438.540-63"));
    }

    #[test]
    fn should_reject_invalid_cpf() {
        assert!(!validate("40743854013"));
    }

    #[test]
    fn should_reject_cpf_with_all_same_digits() {
        assert!(!validate("11111111111"));
    }

    #[test]
    fn should_reject_masked_cpf_without_panic() {
        assert!(!validate("***.104.227-**"));
    }

    #[test]
    fn should_reject_cpf_with_letters() {
        assert!(!validate("407438540AB"));
    }

    #[test]
    fn should_reject_cpf_with_special_characters() {
        assert!(!validate("40743854@63"));
    }

    #[test]
    fn should_generate_valid_cpf() {
        let cpf = generate();
        assert!(validate(&cpf));
    }
}
