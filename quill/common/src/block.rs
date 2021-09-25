/// Returned from `block_get`.
///
/// This is an FFI-safe representation of `Option<u16>`.
#[repr(transparent)]
pub struct BlockGetResult(u32);

impl BlockGetResult {
    pub fn new(block_id: Option<u16>) -> Self {
        let tag = block_id.is_some() as u32;
        let value = (tag << 16) | block_id.unwrap_or_default() as u32;
        Self(value)
    }

    /// Gets the ID of the block.
    pub fn get(self) -> Option<u16> {
        if self.0 >> 16 == 0 {
            None
        } else {
            Some(self.0 as u16)
        }
    }

    pub fn to_u32(&self) -> u32 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn block_get_result_some() {
        let result = BlockGetResult::new(Some(311));
        assert_eq!(result.get(), Some(311));
    }

    #[test]
    fn block_get_result_some_all_bits_set() {
        let result = BlockGetResult::new(Some(u16::MAX));
        assert_eq!(result.get(), Some(u16::MAX));
    }

    #[test]
    fn block_get_result_none() {
        let result = BlockGetResult::new(None);
        assert_eq!(result.get(), None);
    }
}
