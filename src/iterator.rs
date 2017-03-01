/// Iterates a `u64` value in five bit chunks.
///
/// Note that these chunks are ordered from least significant to most, meaning
/// you'll have to reverse them before feeding them to the average 32 bit encoding
/// algorithm.
pub struct FiveBitIterator {
    source: Option<u64>,
    bits_per_chunk: usize,
    shift: usize,
}

impl FiveBitIterator {
    pub fn new(source: u64) -> FiveBitIterator {
        use std::mem;
        let bits_per_chunk = 5;
        FiveBitIterator {
            source: Some(source),
            bits_per_chunk: bits_per_chunk,
            shift: (mem::size_of::<u64>() * 8) - bits_per_chunk,
        }
    }
}

impl Iterator for FiveBitIterator {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        match self.source {
            None => None,
            Some(n) => {
                let chunk = n << self.shift >> self.shift;
                self.source = match n >> self.bits_per_chunk {
                    0 => None,
                    n => Some(n),
                };
                Some(chunk as u8)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use iterator::FiveBitIterator;

    #[test]
    fn zero_returns_zero() {
        let input = 0;
        let expected = &[0];
        let actual: Vec<_> = FiveBitIterator::new(input).collect();

        assert_eq!(expected, &*actual);
    }

    #[test]
    fn values_below_32_return_single_values() {
        let input = 31;
        let expected = &[31];
        let actual: Vec<_> = FiveBitIterator::new(input).collect();

        assert_eq!(expected, &*actual);
    }

    #[test]
    fn thirty_two_becomes_zero_one() {
        let input = 32;
        let expected = &[0, 1];
        let actual: Vec<_> = FiveBitIterator::new(input).collect();

        assert_eq!(expected, &*actual);
    }

    #[test]
    fn large_values_return_correct_values() {
        let input = 4546;
        let expected = &[2, 14, 4];
        let actual: Vec<_> = FiveBitIterator::new(input).collect();

        assert_eq!(expected, &*actual);
    }
}
