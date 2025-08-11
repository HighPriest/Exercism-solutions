pub fn square_of_sum(n: u32) -> u32 {
    
    //METHOD 1
    /*
    (1..=n)
    .fold(0, |result, value| {
        result + value
    }).pow(2)*/

    // METHOD 2
    (1..=n)
    .sum::<u32>().pow(2)

    // METHOD 3
    /*(
        (n * 
            (n + 1)
        )
        / 2
    ).pow(2)*/
}

pub fn sum_of_squares(n: u32) -> u32 {
    // METHOD 1
    (1..=n)
    .fold(0, |result, value|{
        result + (value).pow(2)
    })

    // METHOD 2
    /*n * (n + 1) * (2 * n + 1) / 6*/
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}
