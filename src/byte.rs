use crate::number::some_number;

/// Creates a new byte with a random value.
///
/// Creates a new byte as a `u8` using the `number::some_number()` function.
/// Uses the rand crate to generate the value.
///
/// # Examples
///
/// Basic usage:
/// ```
/// let b = byte::some_byte();
/// ```
pub fn some_byte() -> u8 {
    some_number()
}

/// Creates a vec with random bytes of length `bound: usize`
///
/// Creates a new vec of length `bound: usize` containing random bytes.
/// Internally, this method calls the `byte::some_byte()` function to generate each random byte.
///
/// # Examples
///
/// Basic usage:
/// ```
/// // Create vector of 128 bytes
/// let b_vec = byte::some_byte_vector(128);
/// ```
pub fn some_byte_vector(bound: usize) -> Vec<u8> {
    let mut vec = Vec::new();
    for _ in 0..bound {
        vec.push(some_number())
    }
    vec
}

#[cfg(test)]
mod tests {
    use crate::number::*;
    use super::*;

    #[test]
    fn can_create_random_byte() {
        let _byte = some_byte();
    }

    #[test]
    fn can_create_random_byte_vector() {
        let bound = some_number_between(1, 1024);
        let byte_vector = some_byte_vector(bound);
        assert_eq!(byte_vector.len(), bound);
    }
}