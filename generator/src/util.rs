/// Calculates the product of all values in the given slice.
pub fn slice_product(slice: &[usize]) -> usize {
    let mut result = 1;
    for val in slice {
        result *= *val;
    }

    result
}

/*
/// Returns the highest value in a slice.
pub fn max_in_slice<O: Ord + Copy>(slice: &[O]) -> O {
    assert!(!slice.is_empty());
    let mut highest = slice[0];
    for val in slice {
        if *val > highest {
            highest = *val;
        }
    }

    highest
}*/

/// Returns the lowest value in a slice.
pub fn min_in_slice<O: Ord + Copy>(slice: &[O]) -> O {
    assert!(!slice.is_empty());
    let mut lowest = slice[0];
    for val in slice {
        if *val < lowest {
            lowest = *val;
        }
    }

    lowest
}
