#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_A: &[T], _B: &[T]) -> Comparison {
    //todo!("Determine if the first list is equal to, sublist of, superlist of or unequal to the second list.");
    if _A == _B {
        return Comparison::Equal;
    } else if is_sublist(_B, _A) {
        return Comparison::Sublist;
    } else if is_sublist(_A, _B) {
        return Comparison::Superlist;
    }
    return Comparison::Unequal;
}

fn is_sublist<T: PartialEq>(main: &[T], sub: &[T]) -> bool {
    // Check if subsequence is longer than supersequence. In this case, we can't have subsequence.
    if sub.len() > main.len() {
        return false;
    }

    // We create a window start. It start at 0 & goes until we can no longer fit whole sublist
    for i in 0..(main.len() - sub.len() + 1) {
        // Then we do a comparison, from window start, to window_start+size of sublist.
        if &main[i..i + sub.len()] == sub {
            // If our sublist exists inside, then we return true
            return true;
        }
    }
    return false;
}
