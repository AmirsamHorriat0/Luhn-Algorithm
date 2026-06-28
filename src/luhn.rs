pub fn luhn_implementation(card_number: &str) -> u64 {
    let mut sum = 0u64;

    // iterate over the string each number turns as a char
    for (index, i) in card_number.chars().rev().enumerate() {
        // chars turn to digits base 10
        let mut to_digit = i.to_digit(10).unwrap() as u64;

        if index % 2 != 0 {
            to_digit *= 2;
            // If a doubled digit exceeds 9, subtract 9 from the digit.
            if to_digit > 9 {
                to_digit -= 9;
            }
        }
        sum += to_digit
    }
    sum
}
