pub mod packed_u9 {
    use serde::de::Error as DeError;
    use serde::de::{SeqAccess, Visitor};
    use serde::export::Formatter;
    use serde::ser::Error as SerError;
    use serde::{Deserializer, Serializer};
    use std::marker::PhantomData;

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u16>, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PackedVisitor(PhantomData<fn() -> u16>);

        impl<'de> Visitor<'de> for PackedVisitor {
            type Value = Vec<u16>;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence of type long with length a multiple of 9")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let len = seq.size_hint().unwrap(); // nbt always knows sequence size
                if len % 9 != 0 {
                    // Invalid sequence length
                    return Err(A::Error::custom("sequence length must be a multiple of 9"));
                }
                let unpacked_len = len * 64 / 9;

                let mut u9_array: Vec<u16> = Vec::with_capacity(unpacked_len);

                let mut container: Option<u64> = seq.next_element()?.map(|x: i64| x as u64); // We checked the length
                let mut shift = 0;
                for _elem in 0..unpacked_len {
                    // For every element (u9)

                    // unwrapping here is safe, as this can only fail if there is an implementation error in this algorithm
                    // or in the SeqAccess because we checked the sequence length
                    let mut element: u16 = ((container.unwrap() >> shift) & 0x1FF) as u16;
                    shift += 9;

                    if shift >= 64 {
                        // Take next container
                        container = seq.next_element()?.map(|x: i64| x as u64);

                        if shift > 64 {
                            // We have some bits left to get from the next container

                            // same here with the unwrapping
                            element |= ((container.unwrap() << -(shift - 64 - 9)) & 0x1FF) as u16;
                        }

                        shift -= 64;
                    }

                    u9_array.push(element);
                }

                debug_assert_eq!(container, None);
                debug_assert_eq!(shift, 0);

                Ok(u9_array)
            }
        }

        deserializer.deserialize_seq(PackedVisitor(PhantomData))
    }

    pub fn serialize<S>(u9_array: &[u16], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if u9_array.len() % 64 != 0 {
            // Invalid array length
            return Err(S::Error::custom("array length must be a multiple of 64"));
        }

        let packed_iter = (0..u9_array.len() * 9 / 64) // iterate through each resulting u64
            .map(|i| {
                (
                    i / 9 * 64, // u9_array_offset; every 64 u9 the u64 boundary is aligned with the u9 boundary again -> one section. each section is 9 u64 long
                    i % 9, // container_index; index of the current container in this specific section
                )
            })
            .map(|(u9_array_offset, container_index)| {
                (0..8) // every u64 (partially) contains 8 u9
                    .map(|i| {
                        (
                            i + container_index * 7 + u9_array_offset, // u9_array index; times 7 because the u9 indices need to overlap
                            (i as isize) * 9 - container_index as isize, // amount of shift left (negative means shift right)
                        )
                    })
                    .map(|(u9_array_index, shift_left)| {
                        let u9 = u9_array[u9_array_index] as u64;
                        if shift_left < 0 {
                            u9 >> -shift_left as u64
                        } else {
                            u9 << shift_left as u64
                        }
                    })
                    .fold(0, |container, u9| container | u9)
            })
            .map(|container| container as i64);

        nbt::i64_array(packed_iter, serializer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::iter;
    use serde::{Deserialize, Serialize};
    use serde_test::Token;

    #[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
    struct TestPackedU9 {
        #[serde(with = "packed_u9")]
        list: Vec<u16>,
    }

    #[test]
    #[allow(clippy::inconsistent_digit_grouping)]
    fn test_packed_u9_pattern() {
        let data_u64 = iter::repeat(0xAAAA_AAAA_AAAA_AAAAu64 as i64); // 64-bit 0b1010...
        let data_u9 = [0b01_01_01_01_0u16, 0b1_01_01_01_01u16]
            .iter()
            .cloned()
            .cycle(); // corresponding 9-bit pattern

        let unpacked: Vec<u16> = data_u9.take(256).collect();
        let packed: Vec<i64> = data_u64.take(36).collect();

        // Test serde serialization
        let mut tokenized_vec = packed.iter().map(|&x| Token::I64(x)).collect();

        let mut tokenized_sequence = Vec::new();
        tokenized_sequence.push(Token::Struct {
            name: "TestPackedU9",
            len: 1,
        });
        tokenized_sequence.push(Token::Str("list"));
        // see https://github.com/PistonDevelopers/hematite_nbt/pull/52
        tokenized_sequence.push(Token::TupleStruct {
            name: "__hematite_nbt_i64_array__",
            len: 36,
        });
        tokenized_sequence.append(&mut tokenized_vec);
        tokenized_sequence.push(Token::TupleStructEnd);
        tokenized_sequence.push(Token::StructEnd);

        let test_object = TestPackedU9 { list: unpacked };

        serde_test::assert_tokens(&test_object, tokenized_sequence.as_slice())
    }

    #[test]
    #[allow(clippy::inconsistent_digit_grouping)] // Sorry clippy but grouping by 9 bits makes sense here
    fn test_packed_u9_order() {
        let data_u64 = [
            // this repeats every 9 u64...
            0b0_001000000_000100000_000010000_000001000_000000100_000000010_000000001u64,
            0b00_000100000_000010000_000001000_000000100_000000010_000000001_01000000u64,
            0b000_000010000_000001000_000000100_000000010_000000001_010000000_0010000u64,
            0b0000_000001000_000000100_000000010_000000001_010000000_001000000_000100u64,
            0b01000_000000100_000000010_000000001_010000000_001000000_000100000_00001u64,
            0b000100_000000010_000000001_010000000_001000000_000100000_000010000_0000u64,
            0b0000010_000000001_010000000_001000000_000100000_000010000_000001000_000u64,
            0b00000001_010000000_001000000_000100000_000010000_000001000_000000100_00u64,
            0b010000000_001000000_000100000_000010000_000001000_000000100_000000010_0u64,
        ]
        .iter()
        .cloned()
        .map(|x| x as i64)
        .cycle();
        let data_u9 = (0..8).map(|x| 1 << x).cycle(); // corresponding 9-bit pattern

        let unpacked: Vec<u16> = data_u9.take(256).collect();
        let packed: Vec<i64> = data_u64.take(36).collect();

        // Test serde serialization
        let mut tokenized_vec = packed.iter().map(|&x| Token::I64(x)).collect();

        let mut tokenized_sequence = Vec::new();
        tokenized_sequence.push(Token::Struct {
            name: "TestPackedU9",
            len: 1,
        });
        tokenized_sequence.push(Token::Str("list"));
        // see https://github.com/PistonDevelopers/hematite_nbt/pull/52
        tokenized_sequence.push(Token::TupleStruct {
            name: "__hematite_nbt_i64_array__",
            len: 36,
        });
        tokenized_sequence.append(&mut tokenized_vec);
        tokenized_sequence.push(Token::TupleStructEnd);
        tokenized_sequence.push(Token::StructEnd);

        let test_object = TestPackedU9 { list: unpacked };

        serde_test::assert_tokens(&test_object, tokenized_sequence.as_slice())
    }
}
