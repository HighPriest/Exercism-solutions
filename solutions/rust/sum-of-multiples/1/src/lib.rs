pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    // Filter out any zero factors before processing
    let factors = factors.iter().filter(|&&factor| factor != 0);

    // Use iterator methods to calculate the sum of multiples
    (1..limit) // Create an iterator over numbers from 1 to limit-1
        .filter(|&num| 
            factors.clone().any(|&factor| num % factor == 0) // Filter numbers divisible by any non-zero factor
        ) // We don't have to create another iterator & filter, because factors is already of type filter
        .sum() // Sum the filtered numbers
}

/* V1
pub fn sum_of_mul+tiples(limit: u32, factors: &[u32]) -> u32 {
    let total;
    let mut multiples = vec![];

    // Loop through all numbers less than limit
    for num in 1..limit {
        // Check if num is divisible by any of the factors
        let mut is_multiple = false;
        for &factor in factors {
            if num % factor == 0 {
                is_multiple = true;
                break; // No need to check further factors
            }
        }

        // If num is a multiple of any factor, add it to the total
        if is_multiple {
            multiples.push(num);
        }
    }

    // Sum all unique multiples (duplicates are automatically avoided by our approach)
    total = multiples.iter().sum();

    total
} */