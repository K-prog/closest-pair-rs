use crate::utils::*;
use std::cmp::min;

/// Find closest pair of points using brute force algorithm.
///
/// This function compares every possible pair of points to find the closest pair.
/// It's efficient for small sets of points (typically fewer than 30 points).
///
/// # Arguments
///
/// * `points` - Vector of points to analyze
///
/// # Returns
///
/// A tuple containing:
/// * The first point of the closest pair
/// * The second point of the closest pair
/// * The distance between these points as a f32
///
/// # Panics
///
/// * When the input vector is empty
/// * When there's only one point in the vector
/// * When all distances between points are infinite
///
/// # Examples
///
/// ```
/// use closest_pair_rs::utils::Point;
/// use closest_pair_rs::algorithms::closest_pair_brute_force;
///
/// let points = vec![
///     Point { x: 0, y: 0 },
///     Point { x: 3, y: 0 },
///     Point { x: 0, y: 4 }
/// ];
/// let (p1, p2, distance) =  closest_pair_brute_force(&points);
/// assert_eq!(distance, 3.0);
/// ```
pub fn closest_pair_brute_force(points: &[Point]) -> (Point, Point, f32) {
    // Check if points vector is empty
    if points.is_empty() {
        panic!("Cannot find closest pair with empty vector");
    }

    // Check if there's only one point
    if points.len() < 2 {
        panic!("Need at least two points to find closest pair");
    }

    let mut min_dist = f32::INFINITY;
    // Initialize with the first two points
    let mut point1 = points[0];
    let mut point2 = points[1];

    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let dist = eucid_distance(&points[i], &points[j]);
            if dist < min_dist {
                min_dist = dist;
                point1 = points[i];
                point2 = points[j];
            }
        }
    }

    // Einstein was real
    if min_dist == f32::INFINITY {
        panic!("No closest pair found - all distances might be infinite");
    }

    (point1, point2, min_dist)
}

/// Recursive helper function for the divide-and-conquer closest pair algorithm.
///
/// This function implements the core divide-and-conquer strategy by:
/// 1. Dividing points into left and right halves based on x-coordinates
/// 2. Recursively finding closest pairs in each half
/// 3. Checking for closer pairs that might span the dividing line
///
/// # Arguments
///
/// * `xsorted` - Vector of points sorted by x-coordinate
/// * `ysorted` - Vector of points sorted by y-coordinate
///
/// # Returns
///
/// A tuple containing:
/// * The first point of the closest pair
/// * The second point of the closest pair
/// * The distance between these points as a f32
fn rec(xsorted: &[Point], ysorted: &[Point]) -> (Point, Point, f32) {
    let n = xsorted.len();

    if n <= 3 {
        closest_pair_brute_force(xsorted)
    } else {
        let mid_idx = n / 2;
        let midpoint = xsorted[n / 2];

        let xsorted_left = &xsorted[..mid_idx];
        let xsorted_right = &xsorted[mid_idx..];

        let mut ysorted_left = Vec::with_capacity(mid_idx);
        let mut ysorted_right = Vec::with_capacity(n - mid_idx);

        for &point in ysorted {
            if point.x <= midpoint.x {
                ysorted_left.push(point);
            } else {
                ysorted_right.push(point);
            }
        }

        let (p1_left, p2_left, delta_left) = rec(xsorted_left, &ysorted_left);
        let (p1_right, p2_right, delta_right) = rec(xsorted_right, &ysorted_right);

        let (mut p1, mut p2, mut delta) = if delta_left < delta_right {
            (p1_left, p2_left, delta_left)
        } else {
            (p1_right, p2_right, delta_right)
        };

        // Find points in the band
        let mut in_band = Vec::new();
        let midpoint_x = midpoint.x;

        for &point in ysorted {
            // Notice we need to handle unsigned integers carefully
            let delta_u32 = delta as u32;
            let left_bound = midpoint_x.saturating_sub(delta_u32);
            let right_bound = midpoint_x.saturating_add(delta_u32);

            if point.x >= left_bound && point.x <= right_bound {
                in_band.push(point);
            }
        }

        // Check points in the band
        for i in 0..in_band.len() {
            for j in (i + 1)..min(i + 7, in_band.len()) {
                let d = eucid_distance(&in_band[i], &in_band[j]);
                if d < delta {
                    p1 = in_band[i];
                    p2 = in_band[j];
                    delta = d;
                }
            }
        }

        (p1, p2, delta)
    }
}

/// Find the closest pair of points using an optimized divide-and-conquer algorithm.
///
/// This function implements an O(n log n) algorithm for finding the closest pair
/// of points in a set. It's significantly faster than brute force for large inputs.
///
/// The algorithm works by:
/// 1. Sorting points by x and y coordinates
/// 2. Recursively dividing the problem in half
/// 3. Combining results and checking points near the dividing line
///
/// # Arguments
///
/// * `points` - Vector of points to analyze
///
/// # Returns
///
/// A tuple containing:
/// * The first point of the closest pair
/// * The second point of the closest pair
/// * The distance between these points as a f32
///
/// # Panics
///
/// * When the input vector is empty
/// * When there's only one point in the vector
///
/// # Examples
///
/// ```
/// use closest_pair_rs::utils::Point;
/// use closest_pair_rs::algorithms::closest_pair_optimized;
///
/// let points = vec![
///     Point { x: 0, y: 0 },
///     Point { x: 10, y: 10 },
///     Point { x: 5, y: 5 },
///     Point { x: 7, y: 7 }
/// ];
/// let (p1, p2, distance) = closest_pair_optimized(points);
/// // The closest pair should be (5,5) and (7,7) with distance 2√2
/// ```
pub fn closest_pair_optimized(points: Vec<Point>) -> (Point, Point, f32) {
    // Check if points vector is empty
    if points.is_empty() {
        panic!("Cannot find closest pair with empty vector");
    }

    // Check if there's only one point
    if points.len() < 2 {
        panic!("Need at least two points to find closest pair");
    }

    // Sort by x and y coordinates
    let mut xsorted = points.clone();
    xsorted.sort_by(|a, b| a.x.cmp(&b.x));

    let mut ysorted = points;
    ysorted.sort_by(|a, b| a.y.cmp(&b.y));

    rec(&xsorted, &ysorted)
}

/// Find closest pair of points using bit shift packing technique.
///
/// This function uses bit manipulation to pack x and y coordinates into single values,
/// which are then sorted to find closest pair hehe, and is much easier to understand
///
/// # Arguments
///
/// * `points` - Vector of points to analyze
/// * `bits` - Number of bits to use for each coordinate when packing
///
/// # Returns
///
/// A tuple containing:
/// * The first point of the closest pair
/// * The second point of the closest pair
/// * The distance between these points as a f32
///
/// # Panics
///
/// * When the input vector is empty
/// * When there's only one point in the vector
/// * When all distances between points are infinite
///
/// # Examples
///
/// ```
/// use closest_pair_rs::utils::Point;
/// use closest_pair_rs::algorithms::closest_pair_bit_shift;
///
/// let points = vec![
///     Point { x: 0, y: 0 },
///     Point { x: 10, y: 10 },
///     Point { x: 5, y: 5 },
///     Point { x: 7, y: 7 }
/// ];
/// let (p1, p2, distance) = closest_pair_bit_shift(points, 8);
/// // The closest pair should be (5,5) and (7,7) with distance 2√2
/// ```
pub fn closest_pair_bit_shift(points: Vec<Point>, bits: u8) -> (Point, Point, f32) {
    // Check if points vector is empty
    if points.is_empty() {
        panic!("Cannot find closest pair with empty vector");
    }

    // Check if there's only one point
    if points.len() < 2 {
        panic!("Need at least two points to find closest pair");
    }

    let n = points.len();
    let mut min_dist = f32::INFINITY;
    // Initialize with the first two points
    let mut point1 = points[0];
    let mut point2 = points[1];

    // Pack the points into single values
    let mut packed: Vec<u64> = points
        .iter()
        .map(|p| pack_numbers(p.x, p.y, bits))
        .collect();

    // can use unstable sort as we do not care about the order of identical elements, win
    packed.sort_unstable();

    for i in 0..n - 1 {
        let (x1, y1) = unpack_numbers(packed[i], bits);
        let p1 = Point { x: x1, y: y1 };

        for j in packed
            .iter()
            .take(std::cmp::min(n, i + bits as usize + 1))
            .skip(i + 1)
        {
            let (x2, y2) = unpack_numbers(*j, bits);
            let p2 = Point { x: x2, y: y2 };

            let distance = eucid_distance(&p1, &p2);

            if distance < min_dist {
                min_dist = distance;
                point1 = p1;
                point2 = p2;
            }
        }
    }
    // Einstein was real
    if min_dist == f32::INFINITY {
        panic!("No closest pair found - all distances might be infinite");
    }

    (point1, point2, min_dist)
}

#[cfg(test)]
mod closest_pair_optimized_tests {
    use super::*;
    use std::f32;

    #[test]
    fn test_small_set() {
        // Basic test with known distances
        let points = vec![
            Point { x: 0, y: 0 },
            Point { x: 3, y: 0 },
            Point { x: 0, y: 4 },
            Point { x: 10, y: 10 },
        ];

        let (p1, p2, dist) = closest_pair_optimized(points);
        assert_eq!(dist, 3.0);
        assert!(
            (p1.x == 0 && p1.y == 0 && p2.x == 3 && p2.y == 0)
                || (p2.x == 0 && p2.y == 0 && p1.x == 3 && p1.y == 0)
        );
    }

    #[test]
    fn test_single_pair() {
        // Test with just two points
        let points = vec![Point { x: 5, y: 10 }, Point { x: 8, y: 14 }];

        let (_, _, dist) = closest_pair_optimized(points);
        assert!((dist - 5.0).abs() < 0.001); // Distance should be 5.0
    }

    #[test]
    fn test_collinear_points() {
        // Test with points in a straight line
        let points = vec![
            Point { x: 1, y: 1 },
            Point { x: 3, y: 3 },
            Point { x: 5, y: 5 },
            Point { x: 7, y: 7 },
            Point { x: 9, y: 9 },
        ];

        let (_, _, dist) = closest_pair_optimized(points);
        assert!((dist - 2.0 * f32::sqrt(2.0)).abs() < 0.001); // Distance should be 2√2
    }

    #[test]
    fn test_grid_points() {
        // Test with points arranged in a grid
        let mut points = Vec::new();

        // Create a 5x5 grid with points at integer coordinates
        for x in 0..5 {
            for y in 0..5 {
                points.push(Point { x, y });
            }
        }

        let (_, _, dist) = closest_pair_optimized(points);
        assert_eq!(dist, 1.0); // Minimum distance in a grid is 1.0
    }

    #[test]
    fn test_duplicate_points() {
        // Test with duplicate points (should give distance 0)
        let points = vec![
            Point { x: 10, y: 20 },
            Point { x: 30, y: 40 },
            Point { x: 10, y: 20 }, // Duplicate
            Point { x: 50, y: 60 },
        ];

        let (_, _, dist) = closest_pair_optimized(points);
        assert_eq!(dist, 0.0);
    }

    #[test]
    fn test_large_range() {
        // Test with points spread over a large range
        let points = vec![
            Point { x: 0, y: 0 },
            Point { x: 10000, y: 10000 },
            Point { x: 20000, y: 20000 },
            Point { x: 20005, y: 20005 }, // Closest to the previous point
        ];

        let (_, _, dist) = closest_pair_optimized(points);
        assert!((dist - 5.0 * f32::sqrt(2.0)).abs() < 0.001); // Should be 5√2
    }

    #[test]
    fn test_random_points() {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let bits = 31;

        // Generate 50000 random points
        let mut points = Vec::new();
        for _ in 0..50000 {
            points.push(Point {
                x: rng.gen_range(0..(u32::pow(2, bits))),
                y: rng.gen_range(0..(u32::pow(2, bits))),
            });
        }

        // Run closest pair algorithm
        let (_, _, dist) = closest_pair_optimized(points.clone());

        // Compare with brute force result for validation
        let (_, _, bf_dist) = closest_pair_brute_force(&points);

        // Distances should match, points acn be different
        assert!(dist == bf_dist);
    }

    #[test]
    #[should_panic]
    fn test_empty_vector() {
        // This should panic because we need at least 2 points
        let points: Vec<Point> = Vec::new();
        closest_pair_optimized(points);
    }
}

#[cfg(test)]
mod closest_pair_bit_shift_tests {
    use super::*;
    use std::f32;

    #[test]
    fn test_small_set() {
        // Basic test with known distances
        let points = vec![
            Point { x: 0, y: 0 },
            Point { x: 3, y: 0 },
            Point { x: 0, y: 4 },
            Point { x: 10, y: 10 },
        ];

        let (p1, p2, dist) = closest_pair_bit_shift(points, 8);
        assert_eq!(dist, 3.0);
        assert!(
            (p1.x == 0 && p1.y == 0 && p2.x == 3 && p2.y == 0)
                || (p2.x == 0 && p2.y == 0 && p1.x == 3 && p1.y == 0)
        );
    }

    #[test]
    fn test_single_pair() {
        // Test with just two points
        let points = vec![Point { x: 5, y: 10 }, Point { x: 8, y: 14 }];

        let (_, _, dist) = closest_pair_bit_shift(points, 8);
        assert!((dist - 5.0).abs() < 0.001); // Distance should be 5.0
    }

    #[test]
    fn test_collinear_points() {
        // Test with points in a straight line
        let points = vec![
            Point { x: 1, y: 1 },
            Point { x: 3, y: 3 },
            Point { x: 5, y: 5 },
            Point { x: 7, y: 7 },
            Point { x: 9, y: 9 },
        ];

        let (_, _, dist) = closest_pair_bit_shift(points, 8);
        assert!((dist - 2.0 * f32::sqrt(2.0)).abs() < 0.001); // Distance should be 2√2
    }

    #[test]
    fn test_grid_points() {
        // Test with points arranged in a grid
        let mut points = Vec::new();

        // Create a 5x5 grid with points at integer coordinates
        for x in 0..5 {
            for y in 0..5 {
                points.push(Point { x, y });
            }
        }

        let (_, _, dist) = closest_pair_bit_shift(points, 8);
        assert_eq!(dist, 1.0); // Minimum distance in a grid is 1.0
    }

    #[test]
    fn test_duplicate_points() {
        // Test with duplicate points (should give distance 0)
        let points = vec![
            Point { x: 10, y: 20 },
            Point { x: 30, y: 40 },
            Point { x: 10, y: 20 }, // Duplicate
            Point { x: 50, y: 60 },
        ];

        let (_, _, dist) = closest_pair_bit_shift(points, 8);
        assert_eq!(dist, 0.0);
    }

    #[test]
    fn test_large_range() {
        // Test with points spread over a large range
        let points = vec![
            Point { x: 0, y: 0 },
            Point { x: 10000, y: 10000 },
            Point { x: 20000, y: 20000 },
            Point { x: 20005, y: 20005 }, // Closest to the previous point
        ];

        let (_, _, dist) = closest_pair_bit_shift(points, 16);
        assert!((dist - 5.0 * f32::sqrt(2.0)).abs() < 0.001); // Should be 5√2
    }

    #[test]
    fn test_random_points() {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let bits = 31;
        // Generate 50000 random points
        let mut points = Vec::new();
        for _ in 0..50000 {
            points.push(Point {
                x: rng.gen_range(0..(u32::pow(2, bits))),
                y: rng.gen_range(0..(u32::pow(2, bits))),
            });
        }

        // Run closest pair algorithm
        let (_, _, dist) = closest_pair_bit_shift(points.clone(), 32);

        // Compare with brute force result for validation
        let (_, _, bf_dist) = closest_pair_brute_force(&points);

        // Distances should match, points acn be different
        assert!(dist == bf_dist);
    }

    #[test]
    #[should_panic]
    fn test_empty_vector() {
        // This should panic because we need at least 2 points
        let points: Vec<Point> = Vec::new();
        closest_pair_bit_shift(points, 8);
    }
}
