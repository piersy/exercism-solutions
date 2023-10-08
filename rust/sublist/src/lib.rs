#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if _first_list == _second_list {
        return Comparison::Equal;
    }
    let short;
    let long;
    if _first_list.len() > _second_list.len() {
        long = _first_list;
        short = _second_list;
    } else {
        short = _first_list;
        long = _second_list;
    }
    'outer: for i in 0..long.len() {
        // Not enough space left
        if long.len() - i < short.len() {
            return Comparison::Unequal;
        }
        for j in 0..short.len() {
            if long[i + j] != short[j] {
                continue 'outer;
            }
        }
        if short.len() == long.len() {
            return Comparison::Equal;
        } else if short == _first_list {
            return Comparison::Sublist;
        } else if short == _second_list {
            return Comparison::Superlist;
        }
    }

    Comparison::Unequal
}
