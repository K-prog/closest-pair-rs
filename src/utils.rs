/// A 2D point with unsigned integer coordinates.
#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

/// Packs two positive numbers into a single number using bit manipulation.
///
/// This function takes two positive integers and combines them into a single value
/// by using the specified number of bits for each number. The first number is shifted
/// left and then combined with the second number.
///
/// # Arguments
///
/// * `num1` - First positive integer to pack
/// * `num2` - Second positive integer to pack
/// * `bits` - Number of bits to use for each number
///
/// # Returns
///
/// A u64 containing both numbers packed together
///
/// # Examples
///
/// ```
/// use closest_pair_rs::utils::*;
/// 
/// let packed = pack_numbers(123, 456, 16);
/// assert_eq!(unpack_numbers(packed, 16), (123, 456));
/// ```
pub fn pack_numbers(num1: u32, num2: u32, bits: u8) -> u64 {

    let mask = (1u64 << bits) - 1;
    
    // handling of negative numbers
    // commented as this breaks the algorithm ;-;
    // example
    // couldn't find good a way to pack two close points in different quadrants of cartesian plane so they remain close for the main loop to find it 
    // hope that makes sense

    // let n1 = if num1 < 0 {
        // (num1.abs() as u64 ^ mask) + 1
    // } else {
        // num1 as u64
    // };
    
    // let n2 = if num2 < 0 {
        // (num2.abs() as u64 ^ mask) + 1
    // } else {
        // num2 as u64
    // };
    
    ((num1 as u64 & mask) << bits) | (num2 as u64 & mask)
}

/// Calculates the Euclidean distance between two points.
///
/// # Arguments
///
/// * `p1` - The first point
/// * `p2` - The second point
///
/// # Returns
///
/// The Euclidean distance between p1 and p2 as a f32 value.
///
/// # Examples
///
/// ```
/// use closest_pair_rs::utils::*;
/// 
/// let p1 = Point { x: 0, y: 0 };
/// let p2 = Point { x: 3, y: 4 };
/// assert_eq!(eucid_distance(p1, p2), 5.0);
/// ```
pub fn eucid_distance(p1: Point, p2: Point) -> f32 {
    
    let dx = p1.x.abs_diff(p2.x) as f32;
    let dy = p1.y.abs_diff(p2.y) as f32;
    
    (dx * dx + dy * dy).sqrt()
}

/// Unpacks a single number into two positive numbers.
///
/// This function extracts two positive integers that were previously combined
/// using the `pack_numbers` function, with each number using the specified 
/// number of bits.
///
/// # Arguments
///
/// * `packed` - The combined number to unpack
/// * `bits` - Number of bits used for each original number
///
/// # Returns
///
/// A tuple containing the two extracted positive integers (num1, num2)
///
/// # Examples
///
/// ```
/// use closest_pair_rs::utils::*;
/// 
/// let packed = pack_numbers(42, 127, 8);
/// let (a, b) = unpack_numbers(packed, 8);
/// assert_eq!(a, 42);
/// assert_eq!(b, 127);
/// ```
pub fn unpack_numbers(packed: u64, bits: u8) -> (u32, u32) {

    let mask = (1 << bits) - 1;
    // let sign_bit = 1 << (bits - 1);
    
    // Extract numbers
    let num1 = (packed >> bits) & mask;
    let num2 = packed & mask;
    
    // cant handle negative nums, as explained in pack_numbers ;-;

    // let num1 = if (num1 & sign_bit) != 0 {
    //     -((num1 ^ mask) + 1)
    // } else {
    //     num1
    // };
    
    // let num2 = if (num2 & sign_bit) != 0 {
    //     -((num2 ^ mask) + 1)
    // } else {
    //     num2
    // };
    (num1 as u32 , num2 as u32)
}   


#[cfg(test)]
mod packing_unpacking {
    use super::*;

    #[test]
    fn test_basic_packing_unpacking() {
        let num1 = 42u32;
        let num2 = 123u32;
        let bits = 16u8;
        
        let packed = pack_numbers(num1, num2, bits);
        let (unpacked1, unpacked2) = unpack_numbers(packed, bits);
        
        assert_eq!(num1, unpacked1);
        assert_eq!(num2, unpacked2);
    }
    
    #[test]
    fn test_with_large_numbers() {
        let num1 = 65535u32; // 2^16 - 1
        let num2 = 256u32;   // 2^8 
        let bits = 16u8;
        
        let packed = pack_numbers(num1, num2, bits);
        let (unpacked1, unpacked2) = unpack_numbers(packed, bits);
        
        assert_eq!(num1, unpacked1);
        assert_eq!(num2, unpacked2);
    }
    
    #[test]
    fn test_with_different_bit_sizes() {
        // Test with 8 bits
        let num1 = 127u32;
        let num2 = 255u32;
        let bits = 8u8;
        
        let packed = pack_numbers(num1, num2, bits);
        let (unpacked1, unpacked2) = unpack_numbers(packed, bits);
        
        assert_eq!(num1, unpacked1);
        assert_eq!(num2, unpacked2);
        
        // Test with 24 bits
        let num1 = 16777215u32; // 2^24 - 1
        let num2 = 12345678u32;
        let bits = 24u8;
        
        let packed = pack_numbers(num1, num2, bits);
        let (unpacked1, unpacked2) = unpack_numbers(packed, bits);
        
        assert_eq!(num1, unpacked1);
        assert_eq!(num2, unpacked2);
    }
    
    #[test]
    fn test_truncation() {
        // Test that values larger than the bit size are truncated
        let num1 = 1000u32;
        let num2 = 2000u32;
        let bits = 8u8; // Only 8 bits, so numbers > 255 will be truncated
        
        let packed = pack_numbers(num1, num2, bits);
        let (unpacked1, unpacked2) = unpack_numbers(packed, bits);
        
        assert_eq!(num1 & 0xFF, unpacked1); // Should be 232 (1000 % 256)
        assert_eq!(num2 & 0xFF, unpacked2); // Should be 208 (2000 % 256)
    }
    
    #[test]
    fn test_zero_values() {
        let num1 = 0u32;
        let num2 = 0u32;
        let bits = 16u8;
        
        let packed = pack_numbers(num1, num2, bits);
        let (unpacked1, unpacked2) = unpack_numbers(packed, bits);
        
        assert_eq!(num1, unpacked1);
        assert_eq!(num2, unpacked2);
        assert_eq!(packed, 0);
    }
    
    #[test]
    fn test_bit_boundary() {
        // Test packing at the maximum bit boundary
        let num1 = u32::MAX;  // A large number
        let num2 = u32::MAX;  // A large number
        let bits = 32u8;      // Maximum 32 bits for u32
        
        let packed = pack_numbers(num1, num2, bits);
        let (unpacked1, unpacked2) = unpack_numbers(packed, bits);
        
        assert_eq!(num1, unpacked1);
        assert_eq!(num2, unpacked2);
    }
}

mod eucid_distance {
    use super::*;

    #[test]
    fn test_zero_distance() {
        let p1 = Point { x: 0, y: 0 };
        let p2 = Point { x: 0, y: 0 };
        assert_eq!(eucid_distance(p1, p2), 0.0);
    }

    #[test]
    fn test_horizontal_distance() {
        let p1 = Point { x: 0, y: 0 };
        let p2 = Point { x: 3, y: 0 };
        assert_eq!(eucid_distance(p1, p2), 3.0);
    }

    #[test]
    fn test_vertical_distance() {
        let p1 = Point { x: 0, y: 0 };
        let p2 = Point { x: 0, y: 4 };
        assert_eq!(eucid_distance(p1, p2), 4.0);
    }

    #[test]
    fn test_pythagorean_triple() {
        let p1 = Point { x: 0, y: 0 };
        let p2 = Point { x: 3, y: 4 };
        assert_eq!(eucid_distance(p1, p2), 5.0);
    }

    #[test]
    fn test_reverse_direction() {
        let p1 = Point { x: 5, y: 5 };
        let p2 = Point { x: 2, y: 1 };
        let distance = eucid_distance(p1, p2);
        assert_eq!(distance, 5.0);
    }

    #[test]
    fn test_large_numbers() {
        let p1 = Point { x: 1000, y: 2000 };
        let p2 = Point { x: 4000, y: 6000 };
        let expected = ((3000.0_f32 * 3000.0) + (4000.0_f32 * 4000.0)).sqrt();
        assert_eq!(eucid_distance(p1, p2), expected);
    }
}