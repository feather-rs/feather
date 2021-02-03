//! Assorted utilities not directly related to Minecraft/Feather.

/// Swap-removes an item from a vector by equality.
pub fn vec_remove_item<T: PartialEq>(vec: &mut Vec<T>, item: &T) {
    let index = vec.iter().position(|x| x == item);
    if let Some(index) = index {
        vec.swap_remove(index);
    }
}
