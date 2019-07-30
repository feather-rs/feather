/// Calculates the product of all values in the given slice.
pub fn slice_product(slice: &[usize]) -> usize {
    let mut result = 1;
    for val in slice {
        result *= *val;
    }

    result
}
