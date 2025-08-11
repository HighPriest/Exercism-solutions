pub struct Luhn {
    code: String,
}

impl Luhn {
    pub fn is_valid(&self) -> bool {
        // let code = code_.replace(' ', "");
        let code = self.code.chars().filter(|&c| c != ' ').rev();
        // Create an iterator over the original string, without making a copy of it.

        if code.clone() // With clone, we are creating another iterator with parameters of the previous one. This seems great, because we are not modifying the original string, which is of arbitrary length.
            .any(|c| !c.is_ascii_digit()) { // Check if there are no random non-digit characters in our code
            return false;
        }

        let code = code.filter_map(|c| c.to_digit(10)); // Convert a string of characters into a string of u32. Shadow original variable.
        if code.clone().count() <= 1 {
            return false; // Take note of this pattern. Breaking out of loops early
        }

        (
            code.clone().step_by(2) // Iterate over every uneven value. These don't require multiplication
                .sum::<u32>() // Get sum
            + code.skip(1).step_by(2)// Iterate over every even value. These require manipulation
                .map(|n| n * 2) // Apply the multiplication rule
                .map(|x| if x > 9 { x - 9 } else { x }) // Apply the teens reduction rule (sum of digits, or just -9)
                .sum::<u32>() // Sum resultant value
        ) % 10 == 0 // Check if the last digit is 0 (by modulo 10) & immediately return result
    }
}

/// Here is the example of how the From trait could be implemented
/// for the &str type. Naturally, you can implement this trait
/// by hand for every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
impl From<&str> for Luhn {
    fn from(input: &str) -> Self {
        Luhn {
            code: input.to_string()
        }
    }
}

impl From<String> for Luhn {
    fn from(input: String) -> Self {
        Luhn {
            code: input.to_string()
        }
    }
}

impl From<u8> for Luhn {
    fn from(input: u8) -> Self {
        Luhn {
            code: input.to_string()
        }
    }
}

impl From<u16> for Luhn {
    fn from(input: u16) -> Self {
        Luhn {
            code: input.to_string()
        }
    }
}

impl From<u32> for Luhn {
    fn from(input: u32) -> Self {
        Luhn {
            code: input.to_string()
        }
    }
}

impl From<u64> for Luhn {
    fn from(input: u64) -> Self {
        Luhn {
            code: input.to_string()
        }
    }
}

impl From<usize> for Luhn {
    fn from(input: usize) -> Self {
        Luhn {
            code: input.to_string()
        }
    }
}