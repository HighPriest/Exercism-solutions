/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
        let code = code.replace(" ", "");  // ! Spaces are allowed in the input, but SHOULD be stripped before checking.
                                                            // This needs to be done, because code with space is valid, but other characters are not
        if code.len() > 1 { // ! Strings of length 1 or less are not valid
            let mut sum = 0;

            for (i, c) in code.chars().enumerate() { // For loop, closeable with `return`. Using .for_each doesn't allow for early exit
                match c.to_digit(10) {    // Convert & check if the character is a digit
                    None => return false,       // If not a digit, exit conversion
                    Some(digit) => {
                        if (i + code.len()) % 2 != 0 {  // check parity. If we are on position not divisible by 2, don't do multiplication.
                                                                        // This uses code.len() as an offset for calculating whether i is in n%2 place from start.
                                                                        // Imagine it as if code.len would fill a buffer & each i would cause overflow + 1.
                            sum += digit;
                        } else if digit > 4 {           // We passed the non-multiplying stage first, so we can check if we don't overflow 10
                            sum += 2 * digit - 9;
                        } else {                        // When we are on position divisible by 2, do multiplication
                            sum += 2 * digit;
                        }
                    }
                }
            };

            // This is pseudo-code present on wikipedia, used for calculating the check digit!
            // Wikipedia pesudo-code is wrong!
            // let parity_digit = (10 - (sum % 10)) % 10;
            // code.chars().last().unwrap().to_digit(10).unwrap() == parity_digit
            // code.ends_with((parity_digit).to_string().chars().last().unwrap())
            sum % 10 == 0
        } else {
            false // return false if the code is shorter than 2 characters
        }
        
}
