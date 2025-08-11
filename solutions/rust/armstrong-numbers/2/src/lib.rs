pub fn is_armstrong_number(num: u32) -> bool {
    // Get digits for further processing
    let digits: Vec<u32> = num
                            .to_string()
                            .chars()
                            .map(|d| d.to_digit(10).unwrap())
                            .collect();

    // Store length
    let num_digits = digits.len() as u32;

    // Calculate Armstrong sum
    let sum: u32 = digits
                    .iter()
                    .fold(0, |s, x| x.pow(num_digits) + s);

    // Return
    sum == num
}