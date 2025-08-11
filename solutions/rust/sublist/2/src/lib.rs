#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first: &[T], second: &[T]) -> Comparison {
    // Check if the first list could be a superlist
    let superlist = second.is_empty() || first.windows(second.len()).any(|x| x == second);
    // Check if the second list could be a sublist
    let sublist = first.is_empty() || second.windows(first.len()).any(|x| x == first);
    match (superlist, sublist) {
        (true, true) => Comparison::Equal, // This is only possible, if both are equal.
        (true, false) => Comparison::Superlist,
        (false, true) => Comparison::Sublist,
        (false, false) => Comparison::Unequal,
    }

    // Note: This solution appears superior, to one, which first matches lengths.
    //          The length checking solution has only one advantage, where one value is empty. Because there is no attempt at executing `windows`.
    //          But, executing `windows` on an empty array, cannot be "super slow".
    //          In this solution, we also create two variables, which could increase size of the executed program. But I am not so sure it is worth paying attention to.
}