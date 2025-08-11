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

    //todo!("square of sum of 1...{n}")
}

pub fn sum_of_squares(n: u32) -> u32 {
    (1..=n)
    .fold(0, |result, value|{
        result + (value).pow(2)
    })
    //todo!("sum of squares of 1...{n}")
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
    //todo!("difference between square of sum of 1...{n} and sum of squares of 1...{n}")
}
