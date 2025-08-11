// Note: The best solutions to these tasks: https://exercism.org/tracks/rust/exercises/list-ops/solutions/senekor

/// Yields each item of a and then each item of b
pub fn append<I, J>(_a: I, _b: J) -> impl Iterator<Item = I::Item>
where
    I: Iterator,
    J: Iterator<Item = I::Item>,
{
    _a.chain(_b)
}

/// Combines all items in all nested iterators inside into one flattened iterator
pub fn concat<I>(_nested_iter: I) -> impl Iterator<Item = <I::Item as Iterator>::Item>
where
    I: Iterator,
    I::Item: Iterator,
{
    _nested_iter.flatten()
}

/// Returns an iterator of all items in iter for which `predicate(item)` is true
pub fn filter<I, F>(_iter: I, _predicate: F) -> impl Iterator<Item = I::Item>
where
    I: Iterator,
    F: Fn(&I::Item) -> bool,
{
    let mut result = vec![];
    _iter.for_each(|item| {
        if _predicate(&item) {
            result.push(item);
        }
    });
    result.into_iter()
}

pub fn length<I: Iterator>(_iter: I) -> usize {
    _iter.count()
}

/// Returns an iterator of the results of applying `function(item)` on all iter items
pub fn map<I, F, U>(_iter: I, _function: F) -> impl Iterator<Item = U>
where
    I: Iterator,
    F: Fn(I::Item) -> U,
{
    let mut result = vec![];
    _iter.for_each(|item| {
        result.push(_function(item));
    });
    result.into_iter()
}

pub fn foldl<I, F, U>(mut _iter: I, _initial: U, _function: F) -> U
where
    I: Iterator,
    F: Fn(U, I::Item) -> U,
{
    let mut result = _initial;
    _iter.for_each(|item| {
        result = (_function)(
            std::mem::replace(
                &mut result,
                unsafe { std::mem::zeroed() })
            , item);
    });
    result
}

pub fn foldr<I, F, U>(mut _iter: I, _initial: U, _function: F) -> U
where
    I: DoubleEndedIterator,
    F: Fn(U, I::Item) -> U,
{
    let mut result = _initial;
    _iter.rev().for_each(|item| {
        result = (_function)(
            std::mem::replace(
                &mut result,
                unsafe { std::mem::zeroed() })
            , item);
    });
    result
}

/// Returns an iterator with all the original items, but in reverse order
pub fn reverse<I: DoubleEndedIterator>(_iter: I) -> impl Iterator<Item = I::Item> {
    let mut _iter = _iter;
    std::iter::from_fn(move || _iter.next_back())
}
